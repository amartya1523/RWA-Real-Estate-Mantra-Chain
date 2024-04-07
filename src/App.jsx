import { useEffect, useState } from 'react';
import { ethers } from 'ethers';

// Components
import Navigation from './components/Navigation';
import Search from './components/Search';
import Home from './components/Home';

// ABIs
import RealEstate from './abis/RealEstate.json'
import Escrow from './abis/Escrow.json'

// Config
import config from './config.json';
import home1 from "./metadata/1";
import home2 from "./metadata/2";
import home3 from "./metadata/3";
import home4 from "./metadata/4";
import home5 from "./metadata/5";
import home6 from "./metadata/6";
import home7 from "./metadata/7";
import home8 from "./metadata/8";
import home9 from "./metadata/9";

function App() {
  // State variables for Ethereum provider, Escrow contract, user account, homes data, selected home, and toggle state
  const [provider, setProvider] = useState(null)
  const [escrow, setEscrow] = useState(null)
  const [account, setAccount] = useState(null)
  const [homes, setHomes] = useState([home1, home2, home3, home4, home5, home6, home7, home8, home9])
  const [home, setHome] = useState({})
  const [toggle, setToggle] = useState(false);

  // Function to load data from the Ethereum blockchain
  const loadBlockchainData = async () => {
    // Initialize Ethereum provider
    const provider = new ethers.providers.Web3Provider(window.ethereum)
    setProvider(provider)

    // Get network information
    const network = await provider.getNetwork()

    // Instantiate RealEstate contract with the provider
    const realEstate = new ethers.Contract(config[network.chainId].realEstate.address, RealEstate, provider)
    const totalSupply = await realEstate.totalSupply()
    const homes = []

    // Fetch metadata for  each token and add it to the homes array
    for (var i = 1; i <= totalSupply; i++) {
      const uri = await realEstate.tokenURI(i)
      const response = await fetch(uri)
      const metadata = await response.json()
      homes.push(metadata)
    }

    setHomes(homes)

    // Instantiate Escrow contract with the provider
    const escrow = new ethers.Contract(config[network.chainId].escrow.address, Escrow, provider)
    setEscrow(escrow)

    // Listen for changes in the user's Ethereum account
    window.ethereum.on('accountsChanged', async () => {
      const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
      const account = ethers.utils.getAddress(accounts[0])
      setAccount(account);
    })
  }

  // Load blockchain data when component mounts
  useEffect(() => {
    loadBlockchainData()
  }, [])

  // Function to toggle the visibility of the Home component
  const togglePop = (home) => {
    setHome(home)
    toggle ? setToggle(false) : setToggle(true);
  }

  return (
    <div>
      {/* Navigation and Search components */}
      <Navigation account={account} setAccount={setAccount} />
      <Search />

      <div className='cards__section'>
        {/* Display section title */}
        <h3>Homes For You</h3>
        <hr />

        {/* Display homes as cards */}
        <div className='cards'>
          {homes.map((home, index) => (
            <div className='card' key={index} onClick={() => togglePop(home)}>
              <div className='card__image'>
                <img src={home.image} alt="Home" />
              </div>
              <div className='card__info'>
                <h4>{home.attributes[0].value} OM</h4>
                <p>
                  <strong>{home.attributes[2].value}</strong> bds |
                  <strong>{home.attributes[3].value}</strong> ba |
                  <strong>{home.attributes[4].value}</strong> sqft
                </p>
                <p>{home.address}</p>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Render Home component if toggle is true */}
      {toggle && (
        <Home home={home} provider={provider} account={account} escrow={escrow} togglePop={togglePop} />
      )}
    </div>
  );
}

export default App;