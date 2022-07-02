import { Button, ButtonGroup } from "@mui/material";
import "./App.css";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import { React, useState } from "react";
import * as user from "./GetUser";

export default function App() {
  const theme = createTheme({
    status: {
      danger: "#e53e3e",
    },
    palette: {
      primary: {
        main: "#0971f1",
        darker: "#053e85",
      },
      orange: {
        main: "#ED7F68",
        contrastText: "#fff",
      },
      black: {
        main: "#000000",
      },
      white: {
        main: "#FFFFFF",
      },
    },
  });

  const [account, setAccount] = useState("");
  const [balances, setBalance] = useState(0);
  const [lockedBalance, setLockedBalance] = useState(0);

  const getUserData = async () => {
    const account = await user.getAccount();
    const balance = await user.getBalance(account);
    const stakes =   await user.getStakes(account);

    setAccount(account);
    setBalance(balance);
    setLockedBalance(stakes.data.total_stakes)
  };

  return (
    <ThemeProvider theme={theme}>
      <NavBar></NavBar>
      <div className="App">
        <header className="App-header">
          <box id="account-container" className="container">
            <h4>account     : {account} </h4>
            <h4>STK balance : {balances.data} </h4>
            <h4>Locked      : {lockedBalance}</h4>
          </box>
          <div id="connect-button-container">
            <Button
              id="connect-button"
              variant="text"
              color="orange"
              onClick={getUserData}
            >
              connect
            </Button>
          </div>
          <h3 id="staking-word">Search Stakes</h3>
          <box className="container">
            <input
              id="pub-adress-input"
              placeholder="Public Address"
              style={{ margin: "0 5px" }}
            />
            <br />
            <Button variant="text" color="orange">
              search
            </Button>
          </box>
          <StakeDisplay></StakeDisplay>
        </header>
      </div>
    </ThemeProvider>
  );
}

function NavBar() {
  return (
    <div className="Navbar">
      <ThemeProvider>
        <ButtonGroup variant="text">
          <Button color="white" size="large">
            Stake
          </Button>
        </ButtonGroup>
      </ThemeProvider>
    </div>
  );
}

function StakeDisplay() {
  return <div></div>;
}
