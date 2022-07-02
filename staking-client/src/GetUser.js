import Axios from "axios";
import Web3 from "web3";

let ABI = "./token,json"

export let baseurl = "http://localhost:8080";

export const getAccount = async () => {
    const accounts = await window.ethereum.request({
      method: "eth_requestAccounts",
    });
    const account = accounts[0];
    return account;
  };

export const getBalance = async (account) => {
  const balance = await Axios.get(`${baseurl}/balances/${account}`).then((res)=>{

    return res;
  })

  return balance;
}

export const getStakes = async (account) => {
  const stakes = await Axios.get(`${baseurl}/stakes/${account}`).then((res)=>{

    console.log(res);

    return res;

  });

  return stakes;
}