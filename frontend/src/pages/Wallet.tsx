import React, { useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Button,
  TextField,
  Grid,
  List,
  ListItem,
  ListItemText,
  ListItemSecondaryAction,
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
} from '@mui/material';
import {
  ContentCopy as CopyIcon,
  Visibility as ViewIcon,
  Add as AddIcon,
} from '@mui/icons-material';

interface Wallet {
  address: string;
  balance: string;
  type: string;
}

const Wallet: React.FC = () => {
  const [wallets, setWallets] = useState<Wallet[]>([
    {
      address: '0x1234...5678',
      balance: '1,234.56',
      type: 'Main',
    },
    {
      address: '0x8765...4321',
      balance: '789.01',
      type: 'Trading',
    },
  ]);

  const [openDialog, setOpenDialog] = useState(false);
  const [newWalletName, setNewWalletName] = useState('');

  const handleCreateWallet = () => {
    if (newWalletName) {
      setWallets([
        ...wallets,
        {
          address: '0x' + Math.random().toString(16).slice(2, 10) + '...',
          balance: '0.00',
          type: newWalletName,
        },
      ]);
      setNewWalletName('');
      setOpenDialog(false);
    }
  };

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 4 }}>
        <Typography variant="h4">Wallet Management</Typography>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => setOpenDialog(true)}
        >
          Create New Wallet
        </Button>
      </Box>

      <Grid container spacing={3}>
        {wallets.map((wallet, index) => (
          <Grid item xs={12} md={6} key={index}>
            <Card>
              <CardContent>
                <Typography variant="h6" gutterBottom>
                  {wallet.type} Wallet
                </Typography>
                <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                  <Typography variant="body1" sx={{ flex: 1 }}>
                    {wallet.address}
                  </Typography>
                  <IconButton size="small">
                    <CopyIcon />
                  </IconButton>
                  <IconButton size="small">
                    <ViewIcon />
                  </IconButton>
                </Box>
                <Typography variant="h5" color="primary">
                  ${wallet.balance}
                </Typography>
              </CardContent>
            </Card>
          </Grid>
        ))}
      </Grid>

      <Dialog open={openDialog} onClose={() => setOpenDialog(false)}>
        <DialogTitle>Create New Wallet</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="Wallet Name"
            fullWidth
            value={newWalletName}
            onChange={(e) => setNewWalletName(e.target.value)}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpenDialog(false)}>Cancel</Button>
          <Button onClick={handleCreateWallet} variant="contained">
            Create
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default Wallet; 