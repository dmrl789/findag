import React from 'react';
import { Box, Typography, Button } from '@mui/material';
import { useNavigate } from 'react-router-dom';

const NotFound: React.FC = () => {
  const navigate = useNavigate();
  return (
    <Box sx={{ textAlign: 'center', mt: 10 }}>
      <Typography variant="h2" color="error" gutterBottom>
        404
      </Typography>
      <Typography variant="h5" gutterBottom>
        Page Not Found
      </Typography>
      <Typography color="textSecondary" sx={{ mb: 4 }}>
        The page you are looking for does not exist or has been moved.
      </Typography>
      <Button variant="contained" onClick={() => navigate('/')}>Go to Dashboard</Button>
    </Box>
  );
};

export default NotFound; 