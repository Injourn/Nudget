import { open, save } from '@tauri-apps/api/dialog';
import callTauri from '../functions/CallTauri';
import { useNavigate } from 'react-router-dom';

function LoadingPage(){
    const navigate = useNavigate()
    return (
        <div className="row">
            <button type="button" className="btn btn-primary" onClick={() => open({multiple: false,
                    
                    filters: [{
                        name: 'Database',
                        extensions: ['db']
                    }]
                    }).then((name) => {callTauri("load_file",{filePath:name}).then(() => navigate("/home"))})}>
                test
            </button>
            <button type="button" className="btn btn-primary" onClick={() => save({
                filters: [{
                    name: 'Database',
                    extensions: ['db']
                }]
            }).then((name) => {callTauri("create_file",{filePath:name}).then(() => navigate("/home"))})}>
                create new
            </button>
        </div>
    )
}

export default LoadingPage;