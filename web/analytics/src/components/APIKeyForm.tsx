import React, { useState } from 'react';

interface APIKeyFormProps {
    onSubmit: (apiKey: string) => void;
}

const APIKeyForm: React.FC<APIKeyFormProps> = ({ onSubmit }) => {
    const [apiKey, setApiKey] = useState('');

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        if (apiKey.trim()) {
            onSubmit(apiKey.trim());
        }
    };

    return (
        <form onSubmit={handleSubmit} className="api-key-form">
            <label htmlFor="apiKey">API Key:</label>
            <input
                type="password"
                id="apiKey"
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                placeholder="Enter your API key"
                required
            />
            <button type="submit">Submit</button>
        </form>
    );
};

export default APIKeyForm;