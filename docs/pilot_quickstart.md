# FinDAG Pilot Quickstart Guide

Welcome to the FinDAG pilot program! This guide will help you deploy, operate, and monitor a FinDAG node or cluster for evaluation and feedback.

---

## Prerequisites
- Docker and docker-compose installed (see [Docker Install Guide](https://docs.docker.com/get-docker/))
- At least 2 CPU cores and 4GB RAM per node
- Network access between nodes (if running a cluster)

## 1. Clone the Repository
```sh
git clone https://github.com/your-org/findag.git
cd findag
```

## 2. Launch a Local Cluster
```sh
docker-compose up --build
```
- Node 1 API: http://localhost:8081
- Node 2 API: http://localhost:8082
- Metrics: http://localhost:9891, http://localhost:9892

## 3. Submit a Test Transaction
```sh
curl -X POST http://localhost:8081/tx \
  -H "Content-Type: application/json" \
  -d '{"from": "fdg1q...", "to": "fdg1q...", "amount": 100, "currency": "USD", "signature": "...", "public_key": "..."}'
```

## 4. Check Node Health and Metrics
- Visit http://localhost:9891/metrics for Prometheus metrics
- Import the provided Grafana dashboard for real-time monitoring

## 5. Asset Management and Governance
- To see supported assets:
  ```sh
  curl http://localhost:8081/assets
  ```
- To propose a new asset, use the governance API (see main docs)

## 6. Logs and Audit
- All sensitive actions are logged to `audit.log` in the container or host volume
- For compliance, export and review logs regularly

## 7. Support & Feedback
- For help, open an issue on GitHub or contact the FinDAG team at support@your-org.com
- Please share your feedback and suggestions!

## 8. Feedback & Support
- Please help us improve! After your pilot, fill out the [Pilot Feedback Form](./pilot_feedback_form.md).
- For bugs or feature requests, open a GitHub issue or email support@your-org.com.
- We review all feedback and will respond promptly.

## 9. Join the Community
- Email: support@your-org.com
- Discord: [link-to-discord]
- Forum: [link-to-forum]
- Stay up to date and connect with other pilot users!

## 10. Support & Issue Triage

- All pilot support requests and feedback are monitored by the FinDAG team.
- Issues and feedback are tracked on our GitHub Issues board and internal Kanban (e.g., GitHub Projects).
- Issues are categorized as: bug, feature request, question, or urgent/blocker.
- We aim to respond to all pilot issues within 24 hours.
- For urgent/blocker issues, please email support@your-org.com with "URGENT" in the subject.
- You can follow issue progress on GitHub or request updates via email/community channels.

Thank you for helping us improve FinDAG!

---

Thank you for piloting FinDAG! 