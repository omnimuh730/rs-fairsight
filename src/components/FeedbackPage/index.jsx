import React from "react";
import { Container, TextField, Button, Typography, Box } from "@mui/material";

const FeedbackPage = () => {
    return (
        <Container maxWidth="sm">
            <Box sx={{ textAlign: "center", mt: 5, mb: 3 }}>
                <Typography variant="h5" gutterBottom>
                    We are pleased to hear from you!
                </Typography>
            </Box>
            <Box component="form" sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
                <TextField label="Name" variant="outlined" fullWidth required />
                <TextField label="Email" type="email" variant="outlined" fullWidth required />
                <TextField label="Message" multiline rows={4} variant="outlined" fullWidth required />
                <Button variant="contained" color="primary" size="large">
                    Submit
                </Button>
            </Box>
        </Container>
    );
};

export default FeedbackPage;
