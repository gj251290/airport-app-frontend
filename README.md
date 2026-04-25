# Airport App — Frontend

Proyecto académico para un sistema de reservas de vuelos.

- **Curso:** Aplicación y Servicios Web 2026-1
- **Universidad:** ITM

Este repositorio contiene el **frontend** del sistema, construido con **Rust + Dioxus 0.7** compilado a WebAssembly, consumiendo la API REST del backend desarrollado en FastAPI.

El diseño sigue el sistema **"The Horizon Flow"** — un enfoque editorial de alta gama con paleta azul autoritaria, glassmorphism y tipografía Inter, inspirado en la estética de terminales aeroportuarios premium.

## Sistema de Diseño

### Paleta de Colores

| Token                         | Valor     | Uso                             |
| ----------------------------- | --------- | ------------------------------- |
| `--primary`                   | `#00429d` | Botones, links activos, acentos |
| `--primary-container`         | `#0a58ca` | Gradientes, hover states        |
| `--background`                | `#f8f9ff` | Fondo general                   |
| `--surface-container-low`     | `#eef4ff` | Secciones, cards secundarias    |
| `--surface-container`         | `#e5eeff` | Hubs de interacción             |
| `--surface-container-highest` | `#d9e3f4` | Cards destacadas                |
| `--on-surface`                | `#121c28` | Texto principal                 |
| `--outline-variant`           | `#c3c6d6` | Ghost borders (15% opacity)     |
| `--error`                     | `#ba1a1a` | Errores críticos                |
| `--tertiary`                  | `#7f2b00` | Urgencias (gate closing)        |

### Principios de Diseño

1. **"No-Line" Rule:** No usar bordes de 1px para separar contenido. Usar cambios de color de superficie.
2. **Glassmorphism:** Navbar y overlays con `backdrop-filter: blur(20px)` y opacidad 80%.
3. **Gradientes:** CTAs principales con gradiente `primary → primary-container` a 135°.
4. **Labels:** Siempre uppercase, weight 500, letter-spacing 0.05em (estilo señalización aeropuerto).
5. **Tonal Layering:** Elevar cards mediante delta de luminancia, no sombras.

## Tecnologías

- **Lenguaje:** Rust
- **Framework UI:** Dioxus 0.7 (WebAssembly)
- **Enrutamiento:** dioxus-router 0.7
- **HTTP Client:** reqwest (wasm32)
- **Estilos:** CSS Puro (sin frameworks CSS)
- **Tipografía:** Inter (Google Fonts)
- **Backend:** FastAPI + PostgreSQL (Neon) — [Repositorio backend](https://github.com/gj251290/airport-app)

## Requisitos Previos

- [Rust](https://rust-lang.org/tools/install/) (última versión estable)
- [Dioxus CLI 0.7](https://dioxuslabs.com/learn/0.7/getting_started):
  ```bash
  cargo install dioxus-cli@0.7
  ```
- [wasm32 target](https://rustwasm.github.io/wasm-pack/book/prerequisites/non-rustup-setups.html):
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- Backend corriendo localmente (ver repositorio del backend)
- Navegador moderno con soporte para WebAssembly

## Configuración Inicial

### 1. Clonar el repositorio

```bash
git clone https://github.com/gj251290/airport-app-frontend.git
cd airport-app-frontend
```

### 2. Configurar variables de entorno

Copia el archivo de ejemplo y ajusta los valores:

```bash
cp .env.example .env
```

Edita `.env`:

```env
# URL base del backend FastAPI
API_BASE_URL=http://127.0.0.1:8000

# Clave para almacenar el token en localStorage
JWT_STORAGE_KEY=airport_app_token
```

> El archivo `.env` está ignorado en `.gitignore` y **no debe subirse al repositorio**.

### 3. Compilar y ejecutar

```bash
# Modo desarrollo (hot reload)
dx serve

# O compilar para producción
dx build --release
```

El frontend estará disponible en: **http://localhost:8080**

> Asegúrate de que el backend esté corriendo en `http://127.0.0.1:8000` y que CORS esté configurado correctamente.

## Estructura del Proyecto

```
airport-app-frontend/
├── Cargo.toml              # Dependencias del proyecto
├── Dioxus.toml             # Configuración del bundler
├── .env                    # Variables de entorno (no versionar)
├── .env.example            # Plantilla de variables
│
├── assets/                 # Todos los estilos e imágenes (Dioxus 0.7)
│   ├── main.css            # Estilos globales + reset + utilidades
│   ├── variables.css       # Variables CSS: colores, spacing, tipografía, sombras
│   │
│   ├── components/         # Estilos por componente
│   │   ├── layout.css
│   │   ├── navbar.css
│   │   ├── sidebar.css
│   │   ├── table.css
│   │   ├── form.css
│   │   ├── modal.css
│   │   ├── card.css
│   │   ├── timeline.css
│   │   ├── search.css
│   │   ├── filter.css
│   │   ├── stepper.css
│   │   ├── stats.css
│   │   ├── summary.css
│   │   ├── passengerform.css
│   │   ├── loader.css
│   │   └── alert.css
│   │
│   └── pages/              # Estilos por página
│       ├── login.css
│       ├── landing.css
│       ├── dashboard.css
│       ├── users.css
│       ├── airlines.css
│       ├── airports.css
│       ├── flights.css
│       ├── results.css
│       ├── reservations.css
│       ├── wizard.css
│       ├── confirm.css
│       ├── trips.css
│       └── passengers.css
│
├── public/                 # Archivos estáticos no referenciados por código
│   └── favicon.ico         # (robots.txt, .well-known, etc.)
│
└── src/                    # Solo código Rust
    ├── main.rs             # Punto de entrada
    ├── app.rs              # Router principal, layout raíz, carga de CSS global
    ├── constants.rs        # Configuración global y URLs
    │
    ├── state/              # Estado global (Context API de Dioxus)
    │   ├── mod.rs
    │   ├── auth.rs         # JWT, login/logout
    │   └── app.rs          # Estado general de la aplicación
    │
    ├── api/                # Servicios HTTP por recurso
    │   ├── mod.rs
    │   ├── client.rs       # Cliente HTTP base con headers automáticos
    │   ├── auth.rs
    │   ├── users.rs
    │   ├── airlines.rs
    │   ├── airports.rs
    │   ├── flights.rs
    │   ├── reservations.rs
    │   ├── passengers.rs
    │   └── reservationflights.rs
    │
    ├── models/             # DTOs / Structs (mapeo con Pydantic del backend)
    │   ├── mod.rs
    │   ├── auth.rs
    │   ├── user.rs
    │   ├── airline.rs
    │   ├── airport.rs
    │   ├── flight.rs
    │   ├── reservation.rs
    │   ├── passenger.rs
    │   └── reservationflight.rs
    │
    ├── components/         # Componentes reutilizables (solo .rs)
    │   ├── mod.rs
    │   ├── layout.rs       # Layout admin (sidebar + content area)
    │   ├── navbar.rs       # Navbar público (glassmorphism)
    │   ├── sidebar.rs      # Menú lateral admin
    │   ├── guard.rs        # Guard de rutas protegidas
    │   ├── table.rs        # Tabla genérica con badges de estado
    │   ├── form.rs         # Inputs estilizados con labels uppercase
    │   ├── modal.rs        # Diálogo reutilizable
    │   ├── card.rs         # Card de vuelo (búsqueda)
    │   ├── timeline.rs     # Timeline de itinerario
    │   ├── search.rs       # Barra de búsqueda hero
    │   ├── filter.rs       # Panel de filtros lateral
    │   ├── stepper.rs      # Stepper de progreso del wizard
    │   ├── stats.rs        # Card de estadísticas
    │   ├── summary.rs      # Resumen de reserva sticky
    │   ├── passengerform.rs # Formulario expandible de pasajero
    │   ├── loader.rs       # Spinner animado con CSS puro
    │   └── alert.rs        # Banner de notificación error/éxito
    │
    └── pages/              # Vistas / Páginas (solo .rs)
        ├── mod.rs
        ├── login.rs
        ├── landing.rs
        ├── dashboard.rs
        │
        ├── users/          # CRUD usuarios (admin)
        │   ├── mod.rs
        │   ├── list.rs
        │   ├── create.rs
        │   └── edit.rs
        │
        ├── airlines/       # CRUD aerolíneas (admin)
        │   ├── mod.rs
        │   ├── list.rs
        │   ├── create.rs
        │   └── edit.rs
        │
        ├── airports/       # CRUD aeropuertos (admin)
        │   ├── mod.rs
        │   ├── list.rs
        │   ├── create.rs
        │   └── edit.rs
        │
        ├── flights/        # CRUD vuelos (admin) + búsqueda pública
        │   ├── mod.rs
        │   ├── list.rs
        │   ├── create.rs
        │   ├── edit.rs
        │   └── results.rs
        │
        ├── reservations/   # CRUD reservas + wizard + confirmation + my trips
        │   ├── mod.rs
        │   ├── list.rs
        │   ├── trips.rs
        │   ├── create.rs
        │   ├── edit.rs
        │   ├── wizard.rs
        │   └── confirm.rs
        │
        └── passengers/     # CRUD pasajeros
            ├── mod.rs
            ├── list.rs
            ├── create.rs
            └── edit.rs
```

> **Convención de nombres:** Todos los archivos `.rs` y `.css` usan **una sola palabra** (sin guiones bajos ni concatenaciones). Ej: `passengerform.rs`, `passengerform.css`.

> **Convención de estilos:** CSS puro, sin frameworks. Los estilos globales viven en `assets/main.css` y `assets/variables.css`. Cada componente/página carga su CSS propio con `asset!()` y `document::Stylesheet`. No hay archivos `.css` dentro de `src/`.

## Carga de Estilos en Dioxus 0.7

### Estilos globales

Los estilos globales (`main.css`, `variables.css`) se cargan **una sola vez** en el componente raíz (`src/app.rs`):

```rust
use dioxus::prelude::*;

const GLOBAL_CSS: Asset = asset!("/assets/main.css");
const VARIABLES_CSS: Asset = asset!("/assets/variables.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: VARIABLES_CSS }
        document::Stylesheet { href: GLOBAL_CSS }
        Router::<Route> {}
    }
}
```

### Estilos por componente

Cada componente carga su propio CSS de forma local:

```rust
#[component]
pub fn Navbar() -> Element {
    const NAVBAR_CSS: Asset = asset!("/assets/components/navbar.css");

    rsx! {
        document::Stylesheet { href: NAVBAR_CSS }
        nav { class: "navbar", ... }
    }
}
```

### Reglas de assets

- **No usar `include_str!()`** — `asset!()` genera un path único con hash de contenido para cache busting automático.
- **Hot-reload:** Todos los archivos en `assets/` se recargan en caliente al editarlos durante `dx serve`.
- **Optimización:** CSS se minifica automáticamente al hacer `dx build --release`.
- **`public/`:** Solo para archivos no referenciados por código (favicons, `robots.txt`).

## Flujo Funcional

### Público (sin auth)

1. **Landing:** Hero con search bar, trending destinations, newsletter.
2. **Search Results:** Filtros + lista de flight cards con selección.

### Autenticado (cliente)

3. **My Trips:** Listado de reservas con tabs (Upcoming/Past/Cancelled).
4. **Booking Wizard:**
   - Paso 1: Seleccionar vuelos (ida / ida y vuelta).
   - Paso 2: Ingresar datos de pasajeros (forms expandibles).
   - Paso 3: Confirmar reserva → estado `CONFIRMED`.
5. **Booking Confirmation:** Itinerario, pasajeros, acciones (download, modify, cancel).

### Admin (requiere auth)

6. **Login:** Autenticación con JWT (`admin@admin.com` / `Admin123!`).
7. **Dashboard:** Stats cards, booking volume, revenue share, upcoming departures.
8. **Catálogo:** CRUD de aerolíneas, aeropuertos y vuelos.
9. **Usuarios:** CRUD de usuarios del sistema.
10. **Reservas:** Listado y gestión de reservas.

## Integrantes

| Nombre                                | Rol                            | Responsabilidad Principal                                                                     |
| ------------------------------------- | ------------------------------ | --------------------------------------------------------------------------------------------- |
| **Gerardo Andrés Jiménez Piedrahíta** | Integrador + Motor de Reservas | Proyecto base, router, layout admin, wizard de reservas, dashboard, my trips, confirmation    |
| **Nicolás Josué Grijalba Huertas**    | Catálogo + UI Kit + Landing    | Sistema de diseño CSS, landing page, search results, CRUD airlines/airports, componentes base |
| **Ángel David Gutiérrez Ladino**      | Auth + Users + Flights Admin   | Autenticación JWT, rutas protegidas, CRUD users/flights, login, formularios, loading/error    |

## Backend

El backend del proyecto está en el siguiente repositorio:

[github.com/gj251290/airport-app](https://github.com/gj251290/airport-app)

Asegúrate de levantarlo antes de ejecutar el frontend:

```bash
# En el repositorio del backend
uvicorn app.app:app --reload
```

- **Swagger UI:** http://127.0.0.1:8000/docs
- **ReDoc:** http://127.0.0.1:8000/redoc

## Convenciones de Trabajo

- **Commits:** En español.
- **Código:** En inglés (structs, funciones, variables, nombres de archivo).
- **UI:** En español (textos visibles al usuario).
- **CSS:** Puro, sin frameworks. Variables CSS centralizadas en `assets/variables.css`.
- **Nomenclatura CSS:** Prefijo de módulo para evitar colisiones. Ej: `.airport-table`, `.booking-wizard__step`.
- **Ramas:**
  - `dev` → desarrollo
  - `feat/<nombre-tarea>` → feature branches
  - PR obligatorio hacia `dev` para integrar cambios.

## Licencia

Este proyecto se distribuye bajo la **Licencia MIT**.

**Copyright (c) 2026 — Ángel Gutiérrez, Gerardo Jiménez, Nicolás Grijalba.**

Se otorga permiso por la presente, de forma gratuita, a cualquier persona que obtenga una copia de este software y de los archivos de documentación asociados, para utilizar el Software con fines estrictamente **académicos**, incluyendo sin limitación los derechos de usar, copiar, modificar, fusionar y publicar copias del Software, sujeto a las siguientes condiciones:

1. El aviso de copyright anterior y este aviso de permiso se incluirán en todas las copias o partes sustanciales del Software.
2. **EL SOFTWARE SE PROPORCIONA "TAL CUAL", SIN GARANTÍA DE NINGÚN TIPO, EXPRESA O IMPLÍCITA.**
3. El uso de este software es para fines de aprendizaje en el curso de **Aplicación y Servicios Web 2026-1**.
