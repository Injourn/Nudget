import { Link } from "react-router-dom";


function Sidebar(){

return (
    <div className="offcanvas-lg col-lg-3 offcanvas-start" id="offcanvasResponsive" aria-labelledby="offcanvasResponsiveLabel">
        <div className="offcanvas-header">
            <h6 className="offcanvas-title d-none d-sm-block" id="offcanvas">Menu</h6>
            <button type="button" className="btn-close" data-bs-dismiss="offcanvas" data-bs-target="#offcanvasResponsive" aria-label="Close"></button>
        </div>
        <div className="offcanvas-body px-0">
            <ul className="nav nav-pills flex-column mb-sm-auto mb-0 align-items-start" id="menu">
                <li className="nav-item">
                    <Link to="/" className="nav-link text-truncate">
                        Home
                    </Link>
                </li>
                <li className="nav-item">
                    <Link to="/categories" className="nav-link text-truncate">
                        Categories
                    </Link>
                </li>
                <li className="nav-item">
                    <Link to="/budget" className="nav-link text-truncate">
                        Budgets
                    </Link>
                </li>
            </ul>
        </div>
    </div>
)
}


export default Sidebar;