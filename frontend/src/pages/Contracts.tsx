import React, { useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Button,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Chip,
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  MenuItem,
  Grid,
} from '@mui/material';
import {
  Add as AddIcon,
  PlayArrow as ExecuteIcon,
  Delete as DeleteIcon,
  Code as CodeIcon,
} from '@mui/icons-material';

interface Contract {
  id: string;
  name: string;
  type: 'Clearing' | 'Lending' | 'Compliance';
  status: 'active' | 'pending' | 'completed';
  address: string;
  createdAt: string;
}

const Contracts: React.FC = () => {
  const [contracts, setContracts] = useState<Contract[]>([
    {
      id: '1',
      name: 'BTC Clearing Contract',
      type: 'Clearing',
      status: 'active',
      address: '0x1234...5678',
      createdAt: '2024-03-20',
    },
    {
      id: '2',
      name: 'ETH Lending Pool',
      type: 'Lending',
      status: 'active',
      address: '0x8765...4321',
      createdAt: '2024-03-19',
    },
    {
      id: '3',
      name: 'KYC Verification',
      type: 'Compliance',
      status: 'pending',
      address: '0x9876...5432',
      createdAt: '2024-03-18',
    },
  ]);

  const [openDialog, setOpenDialog] = useState(false);
  const [newContract, setNewContract] = useState({
    name: '',
    type: '',
    code: '',
  });

  const handleCreateContract = () => {
    if (newContract.name && newContract.type && newContract.code) {
      setContracts([
        ...contracts,
        {
          id: (contracts.length + 1).toString(),
          name: newContract.name,
          type: newContract.type as 'Clearing' | 'Lending' | 'Compliance',
          status: 'pending',
          address: '0x' + Math.random().toString(16).slice(2, 10) + '...',
          createdAt: new Date().toISOString().split('T')[0],
        },
      ]);
      setNewContract({ name: '', type: '', code: '' });
      setOpenDialog(false);
    }
  };

  const handleDeleteContract = (id: string) => {
    setContracts(contracts.filter((contract) => contract.id !== id));
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
        return 'success';
      case 'pending':
        return 'warning';
      case 'completed':
        return 'info';
      default:
        return 'default';
    }
  };

  const getTypeColor = (type: string) => {
    switch (type) {
      case 'Clearing':
        return 'primary';
      case 'Lending':
        return 'secondary';
      case 'Compliance':
        return 'error';
      default:
        return 'default';
    }
  };

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 4 }}>
        <Typography variant="h4">Smart Contracts</Typography>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => setOpenDialog(true)}
        >
          Deploy New Contract
        </Button>
      </Box>

      <Grid container spacing={3}>
        {contracts.map((contract) => (
          <Grid item xs={12} key={contract.id}>
            <Card>
              <CardContent>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 2 }}>
                  <Typography variant="h6">{contract.name}</Typography>
                  <Box>
                    <Chip
                      label={contract.type}
                      color={getTypeColor(contract.type)}
                      size="small"
                      sx={{ mr: 1 }}
                    />
                    <Chip
                      label={contract.status}
                      color={getStatusColor(contract.status)}
                      size="small"
                    />
                  </Box>
                </Box>
                <Typography color="textSecondary" gutterBottom>
                  Address: {contract.address}
                </Typography>
                <Typography color="textSecondary" gutterBottom>
                  Created: {contract.createdAt}
                </Typography>
                <Box sx={{ display: 'flex', justifyContent: 'flex-end', mt: 2 }}>
                  <IconButton size="small" sx={{ mr: 1 }}>
                    <ExecuteIcon />
                  </IconButton>
                  <IconButton size="small" sx={{ mr: 1 }}>
                    <CodeIcon />
                  </IconButton>
                  <IconButton
                    size="small"
                    onClick={() => handleDeleteContract(contract.id)}
                  >
                    <DeleteIcon />
                  </IconButton>
                </Box>
              </CardContent>
            </Card>
          </Grid>
        ))}
      </Grid>

      <Dialog
        open={openDialog}
        onClose={() => setOpenDialog(false)}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>Deploy New Smart Contract</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="Contract Name"
            fullWidth
            value={newContract.name}
            onChange={(e) =>
              setNewContract({ ...newContract, name: e.target.value })
            }
          />
          <TextField
            margin="dense"
            label="Contract Type"
            fullWidth
            select
            value={newContract.type}
            onChange={(e) =>
              setNewContract({ ...newContract, type: e.target.value })
            }
          >
            <MenuItem value="Clearing">Clearing Contract</MenuItem>
            <MenuItem value="Lending">Lending Contract</MenuItem>
            <MenuItem value="Compliance">Compliance Contract</MenuItem>
          </TextField>
          <TextField
            margin="dense"
            label="Contract Code"
            fullWidth
            multiline
            rows={4}
            value={newContract.code}
            onChange={(e) =>
              setNewContract({ ...newContract, code: e.target.value })
            }
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpenDialog(false)}>Cancel</Button>
          <Button onClick={handleCreateContract} variant="contained">
            Deploy
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default Contracts;
