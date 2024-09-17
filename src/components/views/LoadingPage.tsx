import { open, save } from '@tauri-apps/api/dialog';
import callTauri from '../../functions/CallTauri';

function LoadingPage(props:any){


    return (
        <div style={{height: '100vh'}} className="d-flex align-items-center justify-content-center">
            <div className="d-grid gap-2 d-md-flex justify-content-md-center">
                <button type="button" className="btn btn-primary" onClick={() => open({multiple: false,  
                        filters: [{
                            name: 'Database',
                            extensions: ['db']
                        }]
                        }).then((name) => {callTauri("load_file",{filePath:name}).then(() => props.setDbLoaded(true))})}>
                    Load File
                </button>
                <button type="button" className="btn btn-primary" onClick={() => save({
                    filters: [{
                        name: 'Database',
                        extensions: ['db']
                    }]
                }).then((name) => {callTauri("create_file",{filePath:name}).then(() => props.setDbLoaded(true))})}>
                    Create New
                </button>
            </div>
        </div>
    )
}

export default LoadingPage;