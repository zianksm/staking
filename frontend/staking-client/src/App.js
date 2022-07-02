import { Button, ButtonGroup } from "@mui/material";
import "./App.css";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import { React, useState } from "react";
import * as user from "./GetUser";
import { Box } from "@mui/system";

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
  const [balances, setBalance] = useState("");
  const [lockedBalance, setLockedBalance] = useState("");
  const [stakes, setStakes] = useState([]);

  const getUserData = async () => {
    const account = await user.getAccount();
    const balance = await user.getBalance(account);
    const stakes = await user.getStakes(account);

    setAccount(account);
    setBalance(balance);
    setLockedBalance(stakes.data.total_stakes);
    setStakes(stakes.data.stakes);
  };

  return (
    <ThemeProvider theme={theme}>
      <NavBar></NavBar>
      <div className="App">
        <header className="App-header">
          <box id="account-container" className="container">
            <h4>account : {account} </h4>
            <h4>STK balance : {balances.data} </h4>
            <h4>Locked : {lockedBalance}</h4>
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
          {stakes.length == 0
            ? "hh"
            : stakes.map((data, _id) => {
                return (
                  <StakeDisplay
                    id={_id + 1}
                    amount={data.amount}
                    claimable={data.claimable}
                    date={ new Date(data.timestamp*1000).toLocaleString()}
                  ></StakeDisplay>
                );
              })}
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

function StakeDisplay(props) {
  return (
    <box className="stake-display">
      <box id="stake-id">
        <h4>Stake {props.id}</h4>
      </box>
      <h5>amount : {props.amount}</h5>
      <h5>claimable : {props.claimable}</h5>
      <h5>date : {props.date}</h5>
    </box>
  );
}
