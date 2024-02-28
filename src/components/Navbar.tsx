

function Navbar(){

    return (
        <nav className="navbar sticky-top navbar-expand-lg bg-body-tertiary">
            <div className="container-fluid">
                <a className="navbar-brand" href="#">Navbar</a>
                <button className="navbar-toggler btn float-end" data-bs-toggle="offcanvas" data-bs-target="#offcanvas" role="button">
                    <i data-bs-toggle="offcanvas" data-bs-target="#offcanvas"> Hello </i>
                </button>
                <div className="collapse navbar-collapse" id="navbarNav">
                    <ul className="navbar-nav">
                        <li className="nav-item">
                            <a href="" className="nav-link active">
                                Home
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    )
}

export default Navbar;