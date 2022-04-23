## History

En Mozilla, el equipo encargado del browser engine Servo (basado en Rust), participaban en el desarrollo del lenguaje Rust.
Los despidos por el Covid disolvieron el equipo de Servo, algunos miembros del equipo eran importantes contribuidores del lenguaje y se temía por su futuro.

## Features

- Performance: similar a C.
- Data race: varios hilos acceden al mismo tiempo a la misma dirección de memoria y alguno la modifica.

### Memory safety

En tiempo de complicación, gracias al sistema de ownership, se evita:

- Punteros que queden colgados.
- Buffer overflow.
- Data race.

Cómo se consigue esto:

- Las variables tienen un dueño (owner).
- Se puede dar ownership de las variables o prestarlas (borrow).
- Cuando el owner está fuera de scope, las variables se liberan/destrullen automáticamente.

