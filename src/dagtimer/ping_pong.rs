use serde::{Serialize, Deserialize};
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use tokio::time::{timeout, sleep, Duration};
use std::sync::Arc;
use rand07::Rng;

#[derive(Serialize, Deserialize, Debug)]
pub enum PingPongMsg {
    Ping { t0: u64 },
    Pong { t0: u64, t1: u64, t2: u64 },
}

pub struct PingPong {
    pub socket: UdpSocket,
}

impl PingPong {
    pub async fn new(bind_addr: &str) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        Ok(Self { socket })
    }

    /// Send a ping to a peer and wait for pong, returning (RTT, offset)
    pub async fn ping_peer(&self, peer_addr: &str) -> std::io::Result<Option<(u64, i64)>> {
        let peer_addr: SocketAddr = peer_addr.parse().unwrap();
        let t0 = now_micros();
        let msg = PingPongMsg::Ping { t0 };
        let buf = bincode::serialize(&msg).unwrap();
        self.socket.send_to(&buf, &peer_addr).await?;

        let mut recv_buf = [0u8; 128];
        // Wait up to 100ms for pong
        if let Ok((len, _)) = timeout(Duration::from_millis(100), self.socket.recv_from(&mut recv_buf)).await? {
            let msg: PingPongMsg = bincode::deserialize(&recv_buf[..len]).unwrap();
            if let PingPongMsg::Pong { t0, t1, t2 } = msg {
                let t3 = now_micros();
                let rtt = t3 - t0;
                let offset = ((t1 as i64 + t2 as i64) / 2) - ((t0 as i64 + t3 as i64) / 2);
                return Ok(Some((rtt, offset)));
            }
        }
        Ok(None)
    }

    /// Listen for incoming pings and respond with pong
    pub async fn listen(&self) -> std::io::Result<()> {
        let mut buf = [0u8; 128];
        loop {
            let (len, peer) = self.socket.recv_from(&mut buf).await?;
            let msg: PingPongMsg = bincode::deserialize(&buf[..len]).unwrap();
            if let PingPongMsg::Ping { t0 } = msg {
                let t1 = now_micros();
                let t2 = now_micros();
                let pong = PingPongMsg::Pong { t0, t1, t2 };
                let pong_buf = bincode::serialize(&pong).unwrap();
                self.socket.send_to(&pong_buf, &peer).await?;
            }
        }
    }

    /// Periodically ping a peer every 4s ±0.5s jitter and feed result to FinDAGTimeManager
    pub async fn periodic_ping<T: Send + Sync + 'static>(
        self: Arc<Self>,
        peer_addr: String,
        time_manager: Arc<T>,
        record_fn: fn(&T, i64, u64),
    ) {
        loop {
            let interval = 4.0 + rand07::thread_rng().gen_range(-0.5, 0.5);
            sleep(Duration::from_secs_f64(interval)).await;
            if let Ok(Some((rtt, offset))) = self.ping_peer(&peer_addr).await {
                // Feed to FinDAGTimeManager using provided function pointer
                record_fn(&time_manager, offset, rtt);
            }
        }
    }
}

fn now_micros() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    now.as_secs() * 1_000_000 + now.subsec_micros() as u64
}

// Example usage (in your main or test):
//
// use std::sync::Arc;
// use dagtimer::findag_time_manager::{FinDAGTimeManager, PeerPing};
//
// #[tokio::main]
// async fn main() {
//     let pingpong = Arc::new(PingPong::new("0.0.0.0:9000").await.unwrap());
//     let time_manager = Arc::new(FinDAGTimeManager::new());
//     // Start listening for pings
//     let pingpong_clone = pingpong.clone();
//     tokio::spawn(async move { pingpong_clone.listen().await.unwrap(); });
//     // Start periodic ping to a peer
//     let peer_addr = "127.0.0.1:9001".to_string();
//     let time_manager_clone = time_manager.clone();
//     tokio::spawn(async move {
//         PingPong::periodic_ping(
//             pingpong.clone(),
//             peer_addr,
//             time_manager_clone,
//             |mgr, offset, rtt| {
//                 mgr.record_peer_ping(PeerPing { offset_us: offset, rtt_us: rtt });
//             },
//         ).await;
//     });
// } 