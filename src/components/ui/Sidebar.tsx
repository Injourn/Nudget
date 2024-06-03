

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
                    <a href="#" className="nav-link text-truncate">
                        Home
                    </a>
                </li>
                <li>
                    <a href="#submenu1" data-bs-toggle="collapse" className="nav-link text-truncate">
                        Dashboard
                    </a>
                </li>
            </ul>
        </div>
    </div>
)
}


export default Sidebar;