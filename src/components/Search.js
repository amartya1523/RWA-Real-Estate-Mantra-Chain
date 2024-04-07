const Search = () => {
    return (
        <header className="header">
            <h1 className="header_title he ">Search it. Explore it. Buy it. 
            <br/>
            Real Estate Markets
            </h1>
            <input
                type="text"
                className="header__search"
                placeholder="Enter an address, neighborhood, city or ZIP code "
            />
        </header>
    );
}

export default Search;