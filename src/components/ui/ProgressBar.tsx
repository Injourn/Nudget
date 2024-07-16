
interface ProgressBarProps{
    color?: string;
    value:number,
}


function ProgressBar(props: ProgressBarProps) {

    return (
        <div className="progress">
            <div className="progress-bar" role="progressbar" style={{ width: props.value + "%", backgroundColor: props.color ?? "blue" }} aria-valuenow={props.value} aria-valuemin={0} aria-valuemax={0}></div>
        </div>
    )
}

export default ProgressBar;