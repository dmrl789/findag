import React, { useState } from 'react';
import {
  Box,
  Typography,
  Card,
  CardContent,
  Grid,
  Button,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Switch,
} from '@mui/material';

const users = [
  { id: 1, name: 'Alice', role: 'Admin', active: true },
  { id: 2, name: 'Bob', role: 'Auditor', active: true },
  { id: 3, name: 'Carol', role: 'User', active: false },
];

const Admin: React.FC = () => {
  const [userList, setUserList] = useState(users);

  const handleToggleActive = (id: number) => {
    setUserList((prev) =>
      prev.map((user) =>
        user.id === id ? { ...user, active: !user.active } : user
      )
    );
  };

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Admin Panel
      </Typography>
      <Grid container spacing={3} sx={{ mb: 4 }}>
        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6">System Status</Typography>
              <Typography color="success.main">All systems operational</Typography>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6">Node Count</Typography>
              <Typography>5</Typography>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6">Pending Updates</Typography>
              <Typography>0</Typography>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
      <Typography variant="h6" gutterBottom>
        User Management
      </Typography>
      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Name</TableCell>
              <TableCell>Role</TableCell>
              <TableCell>Active</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {userList.map((user) => (
              <TableRow key={user.id}>
                <TableCell>{user.name}</TableCell>
                <TableCell>{user.role}</TableCell>
                <TableCell>
                  <Switch
                    checked={user.active}
                    onChange={() => handleToggleActive(user.id)}
                    color="primary"
                  />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
      <Box sx={{ mt: 4 }}>
        <Button variant="contained" color="error">
          Shutdown Node
        </Button>
        <Button variant="outlined" sx={{ ml: 2 }}>
          Restart Services
        </Button>
      </Box>
    </Box>
  );
};

export default Admin; 