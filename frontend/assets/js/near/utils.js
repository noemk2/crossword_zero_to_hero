import { connect, Contract, keyStores, WalletConnection } from 'near-api-js'
import getConfig from './config'

const nearConfig = getConfig(process.env.NODE_ENV || 'development')

// Initialize contract & set global variables
export async function initContract() {
  // Initialize connection to the NEAR testnet
  const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig))

  // Initializing Wallet based Account. It can work with NEAR testnet wallet that
  // is hosted at https://wallet.testnet.near.org
  window.walletConnection = new WalletConnection(near)

  // Getting the Account ID. If still unauthorized, it's just empty string
  window.accountId = window.walletConnection.getAccountId()

  // Initializing our contract APIs by contract name and configuration
  window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
    //aqui colocar nuestro metodos
    // View methods are read only. They don't modify the state, but usually return some value.
    // viewMethods: ['get_greeting'],
    viewMethods: ['get_solution', 'get_puzzle_number', 'guess_solution'],
    // Change methods can modify the state. But you don't receive the returned value when called.
    // changeMethods: ['set_greeting'],
    changeMethods: ['set_solution'],
  })
}

export function logout() {
  window.walletConnection.signOut()
  // reload page
  window.location.replace(window.location.origin + window.location.pathname)
}

export function login() {
  // Allow the current app to make calls to the specified contract on the
  // user's behalf.
  // This works by creating a new access key for the user's account and storing
  // the private key in localStorage.
  window.walletConnection.requestSignIn(nearConfig.contractName)
}

export async function set_solution(message) {
  let response = await window.contract.set_solution({
    args: { message: message }
  })
  return response // nothing
}

export async function get_solution() {
  let greeting = await window.contract.get_solution()
  return greeting //string
}

export async function get_puzzle_number() {
  let greeting = await window.contract.get_puzzle_number()
  return greeting // int
}

export async function guess_solution(message) {
  let response = await window.contract.guess_solution({
    args: { message: message }
  })
  return response // bool
}


// export async function set_greeting(message) {
//   let response = await window.contract.set_greeting({
//     args: { message: message }
//   })
//   return response
// }

// export async function get_greeting() {
//   let greeting = await window.contract.get_greeting()
//   return greeting
// }
export function parseSolutionSeedPhrase(data, gridData) {
  // JavaScript determining what the highest clue number is
  // Example: 10 if there are ten clues, some which have both across and down clues
  let totalClues = Object.keys(data.across).concat(Object.keys(data.down))
    .map(n => parseInt(n))
    .reduce((n, m) => Math.max(n, m));

  let seedPhrase = [];
  // Assume that crossword starts at 1 and goes to totalClues
  for (let i = 1; i <= totalClues; i++) {
    let word = '';
    // If a number has both across and down clues, do across first.
    let iString = i.toString(); // not strictly necessary
    if (data.across.hasOwnProperty(iString)) {
      const answerLength = data.across[i].answer.length;
      for (let j = 0; j < answerLength; j++) {
        word += gridData[data['across'][i].row][data['across'][i].col + j].guess;
      }
      seedPhrase.push(word);
    }
    word = ''; // Clear for items where there's both across and down
    if (data.down.hasOwnProperty(iString)) {
      const answerLength = data.down[i].answer.length;
      for (let j = 0; j < answerLength; j++) {
        word += gridData[data['down'][i].row + j][data['down'][i].col].guess;
      }
      seedPhrase.push(word);
    }
  }
  const finalSeedPhrase = seedPhrase.map(w => w.toLowerCase()).join(' ');
  console.log(`Crossword solution as seed phrase: %c${finalSeedPhrase}`, "color: #00C1DE;");
  return finalSeedPhrase;
}