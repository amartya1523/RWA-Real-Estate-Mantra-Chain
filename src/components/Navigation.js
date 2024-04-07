import { ethers } from 'ethers';
import logo from '../assets/logo.svg';
import { useChain, useWallet } from '@cosmos-kit/react';
import React, { useState } from 'react'; // Import React and useState if not imported

const Navigation = () => {
    const [account, setAccount] = useState(null); // Initialize account state
    const { connect, disconnect, wallet, openView } = useChain("mantrachaintestnet"); // Move useChain outside connectHandler
    const { status, mainWallet } = useWallet("keplr-extension"); // Move useWallet outside connectHandler

    const connectHandler = async () => {
        try {
            const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
            const account = ethers.utils.getAddress(accounts[0]);
            setAccount(account);
        } catch (error) {
            console.error('Error connecting wallet:', error);
        }
    };

    return (
        <nav>
            <div className='hi'>
                <div className='nav__brand'>
                    <img src={logo} alt="Logo" className="nav__logo" />
                    <h1>ReaEsta</h1>
                </div>

                <ul className='nav__links'>
                    <li><a href="#" className="nav__link">Buy</a></li>
                    <li><a href="#" className="nav__link">Rent</a></li>
                    <li><a href="#" className="nav__link">Sell</a></li>
                </ul>

                <button
                    type="button"
                    className='nav__connect'
                    onClick={() => {
                        if (status !== "Connected") {
                            openView();
                        } else {
                            disconnect();
                        }
                    }}>
                    <span> {status !== "Connected" ? "Connect Wallet" : "Disconnect"}</span>
                </button>
            </div>
        </nav>
    );
};

export default Navigation;
