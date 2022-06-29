export const getAccount = async () => {
    const accounts = await window.ethereum.request({
      method: "eth_requestAccounts",
    });
    const account = accounts[0];
    return account;
  };

export const getBalance = async () => {

}

export const getLockedBalance = async () => {
    
}