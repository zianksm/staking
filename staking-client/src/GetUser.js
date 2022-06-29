import Axios from "axios";

let baseurl = "localhost:8080";

export const getAccount = async () => {
    const accounts = await window.ethereum.request({
      method: "eth_requestAccounts",
    });
    const account = accounts[0];
    return account;
  };

export const getBalance = async (account) => {
  const balance = await Axios.get(`http://${baseurl}/balances/${account}`).then((res)=>{

    return res;
  })

  return balance;
}

export const getLockedBalance = async () => {
    
}