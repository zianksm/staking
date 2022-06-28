import { Button, ButtonGroup } from "@mui/material";
import "./App.css";
import { createTheme, ThemeProvider } from '@mui/material/styles';
import React, { useEffect, useState } from "react";



export default function App() {
  
const theme = createTheme({
  status: {
    danger: '#e53e3e',
  },
  palette: {
    primary: {
      main: '#0971f1',
      darker: '#053e85',
    },
    neutral: {
      main: '#ED7F68',
      contrastText: '#fff',
    },
    black:{
      main: "#000000"
    },
    white:{
      main: "#FFFFFF"
    }
  },
});

  return (
    <ThemeProvider theme={theme}>
      <NavBar>

      </NavBar>
    <div className="App">
      <header className="App-header">
          <h3 id="staking-word">Search Stakes</h3>
        <box className="container">
          <input id="pub-adress-input" placeholder="Public Address" style={{margin:"0 5px"}} />
          <br />
          <Button variant="text" color="neutral" >search</Button>
        </box>
      </header>
    </div>
    </ThemeProvider>
  );
}


const NavBar = ()=> {


  return (
   <div className="Navbar">
    <ThemeProvider>

    <ButtonGroup variant="text">
      <Button color="white">Stake</Button>
    </ButtonGroup>
    </ThemeProvider>
   </div>
  )
}



