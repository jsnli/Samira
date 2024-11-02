import { useEffect, useRef } from "react";
import "./index.css";

interface AlertProps {
  message: string[];
}

function Alert({ message }: AlertProps) {
  const alertRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    animate();
  }, [message]);

  function animate() {
    if (
      alertRef.current &&
      message.length > 0 &&
      !alertRef.current.classList.contains("active")
    ) {
      alertRef.current.classList.add("active");
      setTimeout(() => {
        if (alertRef.current) {
          alertRef.current.classList.remove("active");
        }
      }, 3000);
    }
  }
  return (
		<div ref={alertRef} className="alert">
      <div className="message">
				{Array.isArray(message) && message.map((item) => (
					<span>{item}</span>
				))}
      </div>
    </div>
  );
}

export default Alert;
