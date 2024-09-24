import "./index.css";

interface Achievement {
    api_name: string;
    name: string;
    desc: string;
    status: boolean;
}

interface ListProps {
    items: Achievement[];
}

function List({ items }: ListProps) {
    return (
        <>
            <h1>List:</h1>
            <ul className="list">
                {items.map((item, index) => (
                    <li key={index}>
                        <label>
                            <input type="checkbox" checked={item.status} />
                            <div>
                                <span className="name">{item.name}</span>
                                <span className="desc">{item.desc}</span>
                            </div>
                        </label>

                    </li>
                ))}
            </ul>
        </>
    )
}

export default List;
