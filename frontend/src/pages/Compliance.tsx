import React from 'react';
import {
  Box,
  Typography,
  Card,
  CardContent,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Chip,
} from '@mui/material';

const auditLogs = [
  { id: 1, action: 'KYC Approved', user: 'alice', date: '2024-03-20', status: 'success' },
  { id: 2, action: 'AML Check', user: 'bob', date: '2024-03-19', status: 'success' },
  { id: 3, action: 'Transaction Limit Exceeded', user: 'carol', date: '2024-03-18', status: 'warning' },
  { id: 4, action: 'Manual Review', user: 'dave', date: '2024-03-17', status: 'pending' },
];

const Compliance: React.FC = () => {
  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Compliance & Audit
      </Typography>
      <Card sx={{ mb: 4 }}>
        <CardContent>
          <Typography variant="h6">Compliance Overview</Typography>
          <Typography color="textSecondary">
            All users must pass KYC/AML checks. Transactions are monitored for regulatory compliance and audit trails are maintained.
          </Typography>
        </CardContent>
      </Card>
      <Typography variant="h6" gutterBottom>
        Audit Log
      </Typography>
      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Date</TableCell>
              <TableCell>User</TableCell>
              <TableCell>Action</TableCell>
              <TableCell>Status</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {auditLogs.map((log) => (
              <TableRow key={log.id}>
                <TableCell>{log.date}</TableCell>
                <TableCell>{log.user}</TableCell>
                <TableCell>{log.action}</TableCell>
                <TableCell>
                  <Chip
                    label={log.status}
                    color={
                      log.status === 'success'
                        ? 'success'
                        : log.status === 'warning'
                        ? 'warning'
                        : log.status === 'pending'
                        ? 'info'
                        : 'default'
                    }
                    size="small"
                  />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </Box>
  );
};

export default Compliance; 