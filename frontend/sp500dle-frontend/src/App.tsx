import React, { useState } from 'react';
import axios from 'axios';

const App: React.FC = () => {
  const [gameStatus, setGameStatus] = useState('');
  const [guess, setGuess] = useState('');
  const [hints, setHints] = useState<any>(null);

  const startGame = async () => {
    try {
      const response = await axios.post('/api/start');
      setGameStatus(response.data);
    } catch (error) {
      console.error('Error starting game', error);
    }
  };

  const makeGuess = async () => {
    try {
      const response = await axios.post('/api/guess', { ticker: guess });
      setHints(response.data);
    } catch (error) {
      console.error('Error making guess', error);
    }
  };

  return (
    <div>
      <h1>Welcome to SP500dle</h1>
      <button onClick={startGame}>Start Game</button>
      <div>{gameStatus}</div>
      <input
        type="text"
        value={guess}
        onChange={(e) => setGuess(e.target.value)}
        placeholder="Enter stock ticker"
      />
      <button onClick={makeGuess}>Make Guess</button>
      <pre>{hints && JSON.stringify(hints, null, 2)}</pre>
    </div>
  );
};

export default App;
