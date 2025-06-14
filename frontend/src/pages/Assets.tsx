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
} from '@mui/material';
import {
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
} from '@mui/icons-material';

interface Asset {
  id: string;
  name: string;
  type: string;
  value: string;
  status: 'active' | 'pending' | 'frozen';
}

const Asset: React.FC = () => {
  const [assets, setAssets] = useState<Asset[]>([
    {
      id: '1',
      name: 'BTC',
      type: 'Cryptocurrency',
      value: '45,678.90',
      status: 'active',
    },
    {
      id: '2',
      name: 'ETH',
      type: 'Cryptocurrency',
      value: '2,345.67',
      status: 'active',
    },
    {
      id: '3',
      name: 'USDC',
      type: 'Stablecoin',
      value: '100,000.00',
      status: 'active',
    },
  ]);

  const [openDialog, setOpenDialog] = useState(false);
  const [editingAsset, setEditingAsset] = useState<Asset | null>(null);
  const [newAsset, setNewAsset] = useState({
    name: '',
    type: '',
    value: '',
  });

  const handleCreateAsset = () => {
    if (newAsset.name && newAsset.type && newAsset.value) {
      setAssets([
        ...assets,
        {
          id: (assets.length + 1).toString(),
          ...newAsset,
          status: 'active',
        },
      ]);
      setNewAsset({ name: '', type: '', value: '' });
      setOpenDialog(false);
    }
  };

  const handleEditAsset = (asset: Asset) => {
    setEditingAsset(asset);
    setNewAsset({
      name: asset.name,
      type: asset.type,
      value: asset.value,
    });
    setOpenDialog(true);
  };

  const handleUpdateAsset = () => {
    if (editingAsset && newAsset.name && newAsset.type && newAsset.value) {
      setAssets(
        assets.map((asset) =>
          asset.id === editingAsset.id
            ? { ...asset, ...newAsset }
            : asset
        )
      );
      setEditingAsset(null);
      setNewAsset({ name: '', type: '', value: '' });
      setOpenDialog(false);
    }
  };

  const handleDeleteAsset = (id: string) => {
    setAssets(assets.filter((asset) => asset.id !== id));
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
        return 'success';
      case 'pending':
        return 'warning';
      case 'frozen':
        return 'error';
      default:
        return 'default';
    }
  };

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 4 }}>
        <Typography variant="h4">Asset Management</Typography>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => {
            setEditingAsset(null);
            setNewAsset({ name: '', type: '', value: '' });
            setOpenDialog(true);
          }}
        >
          Add New Asset
        </Button>
      </Box>

      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Name</TableCell>
              <TableCell>Type</TableCell>
              <TableCell>Value</TableCell>
              <TableCell>Status</TableCell>
              <TableCell align="right">Actions</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {assets.map((asset) => (
              <TableRow key={asset.id}>
                <TableCell>{asset.name}</TableCell>
                <TableCell>{asset.type}</TableCell>
                <TableCell>${asset.value}</TableCell>
                <TableCell>
                  <Chip
                    label={asset.status}
                    color={getStatusColor(asset.status)}
                    size="small"
                  />
                </TableCell>
                <TableCell align="right">
                  <IconButton
                    size="small"
                    onClick={() => handleEditAsset(asset)}
                  >
                    <EditIcon />
                  </IconButton>
                  <IconButton
                    size="small"
                    onClick={() => handleDeleteAsset(asset.id)}
                  >
                    <DeleteIcon />
                  </IconButton>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>

      <Dialog open={openDialog} onClose={() => setOpenDialog(false)}>
        <DialogTitle>
          {editingAsset ? 'Edit Asset' : 'Add New Asset'}
        </DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="Asset Name"
            fullWidth
            value={newAsset.name}
            onChange={(e) =>
              setNewAsset({ ...newAsset, name: e.target.value })
            }
          />
          <TextField
            margin="dense"
            label="Asset Type"
            fullWidth
            select
            value={newAsset.type}
            onChange={(e) =>
              setNewAsset({ ...newAsset, type: e.target.value })
            }
          >
            <MenuItem value="Cryptocurrency">Cryptocurrency</MenuItem>
            <MenuItem value="Stablecoin">Stablecoin</MenuItem>
            <MenuItem value="Token">Token</MenuItem>
          </TextField>
          <TextField
            margin="dense"
            label="Value"
            fullWidth
            value={newAsset.value}
            onChange={(e) =>
              setNewAsset({ ...newAsset, value: e.target.value })
            }
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpenDialog(false)}>Cancel</Button>
          <Button
            onClick={editingAsset ? handleUpdateAsset : handleCreateAsset}
            variant="contained"
          >
            {editingAsset ? 'Update' : 'Create'}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default Asset;
