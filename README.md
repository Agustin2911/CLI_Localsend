# cli_localsend


Una implementación ligera y rápida por línea de comandos (CLI) inspirada en LocalSend, diseñada para transferir archivos entre dispositivos en la misma red local de forma segura y eficiente.

---

## ⚙️ Explicación Técnica

`cli_localsend` está desarrollado en **Rust** y utiliza una arquitectura de red basada en **Sockets TCP** (Transmission Control Protocol) para garantizar la entrega íntegra de los archivos. 

El flujo de comunicación funciona de la siguiente manera:

1. **Descubrimiento de Dispositivos:** El programa escanea la red local para identificar otros nodos que estén ejecutando la aplicación en modo "escucha".
2. **Handshake y Control:** Antes de transferir un archivo, el emisor y el receptor intercambian metadatos (nombre, tamaño, cantidad de archivos) empaquetados en formato **JSON** utilizando `serde_json`. El receptor debe aceptar explícitamente la transferencia.
3. **Fragmentación (Chunking):** Los archivos grandes no se cargan enteros en la memoria RAM. Se leen y se transmiten en pequeños fragmentos (chunks) de 64 KB. Cada fragmento viaja como una estructura JSON que indica su número de secuencia y si es el fragmento o archivo final, asegurando una reconstrucción perfecta del otro lado.
4. **Configuración Persistente:** Las preferencias del usuario (nombre visible y directorio de descargas) se almacenan de forma segura en los directorios de configuración estándar del sistema operativo (ej. `~/.config/cli_localsend/config.json` en distribuciones Linux).

---

## 🚀 Cómo compilar e instalar

Para compilar este proyecto, necesitás tener [Rust y Cargo instalados](https://www.rust-lang.org/tools/install) en tu sistema.

### Instalación Global (Recomendado)
Para que el comando `cli_localsend` esté disponible en cualquier carpeta de tu terminal, ejecutá el siguiente comando desde la raíz del proyecto:

```bash
cargo install --path .
