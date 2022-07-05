import 'regenerator-runtime/runtime'
import React, { useCallback, useRef, useState } from 'react'
// import Crossword from 'react-crossword-near';
// import Crossword from '@jaredreisinger/react-crossword';
import Crossword from 'react-crossword-near';
import { createGridData, loadGuesses } from "react-crossword-near/dist/es/util";

import sha256 from 'js-sha256'

import './assets/css/global.css'

// import { login, logout, get_greeting, set_greeting } from './assets/js/near/utils'
import { login, logout, get_puzzle_number, get_solution } from './assets/js/near/utils'
import getConfig from './assets/js/near/config'
import { parseSolutionSeedPhrase } from './assets/js/near/utils';


export default function App() {
  const crossword = useRef();
  // use React Hooks to store greeting in component state
  const [greeting, setGreeting] = React.useState()

  const [solution, setSolution] = React.useState()
  // when the user has not yet interacted with the form, disable the button
  // const [buttonDisabled, setButtonDisabled] = React.useState(true)

  // after submitting the form, we want to show Notification
  const [showNotification, setShowNotification] = React.useState(false)

  const [solutionFound, setSolutionFound] = useState("Not correct yet");


  // The useEffect hook can be used to fire side-effects during render
  // Learn more: https://reactjs.org/docs/hooks-intro.html
  React.useEffect(
    () => {
      // get_greeting is in near/utils.js
      get_puzzle_number()
        .then(greetingFromContract => {
          setGreeting(greetingFromContract)
        })

      get_solution().then(res => {
        setSolution(res)
      })
    },
    []
  )

  const data = {
    across: {
      1: {
        clue: 'Native token',
        answer: '????',
        row: 1,
        col: 2,
      },
      3: {
        clue: 'DeFi decentralizes this',
        answer: '???????',
        row: 7,
        col: 0,
      },
    },
    down: {
      1: {
        clue: 'Name of the spec/standards site is _______.io',
        answer: '???????',
        row: 1,
        col: 2,
      },
      2: {
        clue: 'DeFi site on NEAR is ___.finance',
        answer: '???',
        row: 1,
        col: 5,
      },
    },
  };






  const onCrosswordComplete = useCallback(
    async (completeCount) => {
      if (completeCount !== false) {
        let gridData = createGridData(data).gridData;
        loadGuesses(gridData, 'guesses');
        await checkSolution(gridData);
      }
    },
    []
  );


  async function checkSolution(gridData) {
    let seedPhrase = parseSolutionSeedPhrase(data, gridData);
    let answerHash = sha256.sha256(seedPhrase);
    // Compare crossword solution's public key with the known public key for this puzzle
    // (It was given to us when we first fetched the puzzle in index.js)
    if (answerHash === solution) {
      console.log("You're correct!");
      setSolutionFound("Correct!");
    } else {
      console.log("That's not the correct solution. :/");
      setSolutionFound("Not correct yet");
    }
  }


  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) {
    return (
      <main>
        <h1>
          <label
            htmlFor="greeting"
            style={{
              color: 'var(--secondary)',
              borderBottom: '2px solid var(--secondary)'
            }}
          >
            {greeting}
          </label>!
          Welcome crossword: the game
        </h1>
        <p>
          Your contract is storing a greeting message in the NEAR blockchain. To
          change it you need to sign in using the NEAR Wallet. It is very simple,
          just use the button below.
        </p>
        <p>
          Do not worry, this app runs in the test network ("testnet"). It works
          just like the main network ("mainnet"), but using NEAR Tokens that are
          only for testing!
        </p>
        <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
          <button onClick={login}>Sign in</button>
        </p>
      </main>
    )
  }

  return (
    // use React Fragment, <>, to avoid wrapping elements in unnecessary divs
    <>
      <button className="link" style={{ float: 'right' }} onClick={logout}>
        Sign out
      </button>
      <main>
        <h1>
          <label
            htmlFor="greeting"
            style={{
              color: 'var(--secondary)',
              borderBottom: '2px solid var(--secondary)'
            }}
          >
            {/* {greeting} */}
          </label>
          {' '/* React trims whitespace around tags; insert literal space character when needed */}
          Hi {window.accountId}! <br />
          la respuesta es:
        </h1>

        <p>{solution} </p>

        {/* end form  */}

        <div id="crossword-wrapper">

          <h3>Status: {solutionFound}</h3>
          <Crossword
            data={data}
            ref={crossword}
            // onAnswerComplete={saludo_dos} 
            onCrosswordComplete={onCrosswordComplete}
          />



        </div>
      </main>
      {showNotification && <Notification />}
    </>
  )
}

// this component gets rendered by App after the form is submitted
function Notification() {
  const { networkId } = getConfig(process.env.NODE_ENV || 'development')
  const urlPrefix = `https://explorer.${networkId}.near.org/accounts`
  return (
    <aside>
      <a target="_blank" rel="noreferrer" href={`${urlPrefix}/${window.accountId}`}>
        {window.accountId}
      </a>
      {' '/* React trims whitespace around tags; insert literal space character when needed */}
      called method: 'set_greeting' in contract:
      {' '}
      <a target="_blank" rel="noreferrer" href={`${urlPrefix}/${window.contract.contractId}`}>
        {window.contract.contractId}
      </a>
      <footer>
        <div>✔ Succeeded</div>
        <div>Just now</div>
      </footer>
    </aside>
  )
}
