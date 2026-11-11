# Qué probar al instalar DexWordCraft

Instala el APK (`DexWordCraft-…-arm64-v8a.apk` o el de debug de la CI) en un móvil o tableta con
Android 10 o posterior. Si algo falla, instala el APK de debug de la CI y pulsa **Ctrl+Mayús+D**
(o abre su icono «DexWordCraft diagnostics»), copia el informe y pégalo en el issue o el chat. En
el APK de release el informe no está a la vista (craft-android lo deja solo para debug).

Marca cada punto con ✅ o ❌ y anota lo que veas.

## 1. Arranque y cierre (DeX, con ratón)
- [ ] Abre la app desde el escritorio de DeX: aparece en una ventana de unos 1280×800 con la
      pantalla de inicio de WordCraft (Archivo ▸ Inicio).
- [ ] Ciérrala con la X de la ventana y vuelve a abrirla **tres veces seguidas**, sin que se cierre
      sola, se cuelgue o quede en negro.
- [ ] Maximiza, redimensiona y mueve la ventana entre la pantalla del móvil y el monitor: la
      interfaz se redibuja a su tamaño y no se reinicia.
- [ ] El título en la barra de tareas de DeX es «Documento1 - DexWordCraft» (o el nombre del
      archivo abierto) y añade « •» al hacer cambios.
- [ ] Idioma: la interfaz sale en el idioma del sistema (en español si el móvil está en español), y
      un documento nuevo usa el papel de la región (A4 en España).
- [ ] Tema: el claro o el oscuro sigue al del sistema.

## 2. Ratón
- [ ] Clic para poner el cursor, arrastrar para seleccionar, doble clic selecciona una palabra y
      triple clic un párrafo.
- [ ] Clic derecho sobre el texto, una tabla y una imagen: sale el menú contextual.
- [ ] La rueda desplaza las páginas, y Ctrl+rueda hace zoom.
- [ ] Cursor: I sobre el texto, mano sobre los enlaces y flechas de redimensionar en los bordes de
      tablas, imágenes y márgenes de la regla.
- [ ] La cinta (Inicio, Insertar, Diseño…) cambia de pestaña y los botones responden al pasar el
      ratón por encima.

## 3. Teclado físico
- [ ] Escribe: letras, números, `ñ`, acentos (´ + a = á), `@`, `€` con AltGr, y comillas tipográficas.
- [ ] Enter, Retroceso, Supr y Tab; Inicio y Fin; Ctrl+flechas para moverse por palabras; Mayús+flechas
      para seleccionar.
- [ ] Listas: «1. » o «- » al empezar una línea crea una lista; Tab y Mayús+Tab cambian el nivel.
- [ ] Atajos: Ctrl+Z / Ctrl+Y, Ctrl+N, Ctrl+O, Ctrl+S, Ctrl+B, Ctrl+I, Ctrl+U, Ctrl+F (buscar) y
      Ctrl+H (reemplazar).
- [ ] **Copiar y pegar**:
  - [ ] Ctrl+C y Ctrl+V dentro del documento conservan el formato.
  - [ ] Copia texto de otra app (navegador, Notas) y pégalo con Ctrl+V.
  - [ ] Copia texto con formato de DexWordCraft y pégalo en otra app que lo admita (Gmail, Docs): se
        mantienen la negrita y los títulos.
  - [ ] Copia de una página web y pégalo: se conservan los enlaces y el formato.
  - [ ] Ctrl+X corta, y el botón Pegar de la cinta pega lo que hay en el portapapeles del sistema.
  - [ ] Copia una imagen (por ejemplo, de la Galería) y pégala con el botón Pegar: se inserta.
- [ ] Ortografía: escribe una palabra mal; sale subrayada en rojo y el clic derecho ofrece
      sugerencias (en español si el documento está en español).

## 4. Archivos
- [ ] Archivo ▸ Abrir (y Ctrl+O): sale el selector de Android. Abre un `.docx` y comprueba que se ve
      bien (fuentes, tablas, imágenes, encabezados).
- [ ] Abre también un `.odt`, un `.rtf`, un `.doc` antiguo y un `.txt`.
- [ ] Edita el `.docx` y pulsa Ctrl+S: **el cambio aparece en el archivo original** (ábrelo con otra
      app o vuelve a abrirlo).
- [ ] En un documento nuevo, pulsa Ctrl+S (o Archivo ▸ Guardar como): sale el selector para crear el
      archivo; tras elegirlo se guarda y el título cambia al nombre nuevo.
- [ ] Archivo ▸ Exportar ▸ PDF (y otro formato, como ODT o Markdown): pide dónde guardarlo y el
      archivo aparece con su contenido.
- [ ] En el selector de guardar, pulsa Atrás o Cancelar: no se guarda nada y la app sigue normal.
- [ ] Desde «Mis archivos», «Abrir con» un `.docx` → DexWordCraft: se abre en la app.
- [ ] Desde DexFiles, «Abrir» un `.docx`, un `.odt` o un `.txt` con DexWordCraft: se abre en la app
      (la app anuncia a DexFiles las extensiones que abre).
- [ ] Desde otra app, «Compartir» un `.docx` o un `.txt` → DexWordCraft: se abre.
- [ ] En DeX, arrastra un `.docx` desde «Mis archivos» a la ventana: se abre.
- [ ] **Pestañas** (debajo de la cinta): abre dos documentos; cada uno tiene su pestaña y la del que
      se ve está resaltada. Un clic en la otra la muestra; el + crea un documento en blanco.
- [ ] Modifica un documento: su pestaña muestra un punto « •» en lugar de la ×, y la × vuelve al
      pasar el ratón por encima.
- [ ] La × de una pestaña sin cambios la cierra sin preguntar. Con cambios, pregunta Guardar / No
      guardar / Cancelar (también si es la de un documento que no se está viendo: primero lo
      muestra). Clic central sobre una pestaña hace lo mismo que su ×.
- [ ] Con muchos documentos, las pestañas se desplazan en horizontal y la activa queda a la vista.
- [ ] Archivo ▸ Abrir ▸ Recientes: lista los abiertos, y vuelven a abrirse.
- [ ] Insertar ▸ Imágenes: elige una imagen PNG o JPEG y se inserta en el documento.

## 5. Cerrar y recuperar
- [ ] Haz un cambio y pulsa el botón Atrás: pregunta Guardar / No guardar / Cancelar. Cancelar deja
      la app abierta, y No guardar la cierra.
- [ ] Haz un cambio sin guardar y cierra **enseguida** con la X de DeX (que no pregunta). Al volver
      a abrir, WordCraft ofrece recuperar el documento con el cambio (la copia se hace al perder la
      ventana el foco).
- [ ] Cambia una opción (Archivo ▸ Opciones), cierra con la X y vuelve a abrir: la opción se
      mantiene.

## 6. Funciones del sistema
- [ ] Archivo ▸ Imprimir ▸ Imprimir: se abre el diálogo de impresión de Android con la vista previa
      del documento, y «Guardar como PDF» funciona.
- [ ] Un hipervínculo `https://…` del documento (Ctrl+clic) abre el navegador.
- [ ] Ayuda ▸ Buscar actualizaciones: dice que la app la actualiza la tienda (no intenta descargar).
- [ ] Textos en chino, japonés, coreano, árabe, hebreo y tailandés se ven con sus letras (no
      cuadrados ▯).
- [ ] Un `.docx` con Calibri o Times New Roman se ve con una fuente parecida y con los saltos de
      página en su sitio.

## 7. Móvil o tableta (sin DeX)
- [ ] Toca el texto y escribe: sale el teclado en pantalla con predicción, lo escrito va al
      documento y el teclado no tapa la línea que escribes.
- [ ] La interfaz no queda debajo de la barra de estado ni de la de navegación.
- [ ] Gira la pantalla: se adapta sin reiniciarse ni perder el documento.
- [ ] Desplazamiento con el dedo y zoom con dos dedos.
- [ ] Sal a la pantalla de inicio y vuelve: el documento sigue igual.

## 8. Rendimiento
- [ ] Un documento largo (decenas de páginas) se desplaza con fluidez, y escribir al principio no va
      con retraso.
- [ ] El informe de diagnóstico (Ctrl+Mayús+D) dice `renderer wgpu, … (Vulkan…)` o
      `renderer glow`; si dice glow, anótalo.

## Límites conocidos (no son fallos)
- Los documentos se cambian dentro de una sola ventana (con las pestañas), no en ventanas de DeX
  aparte.
- La X de DeX cierra sin preguntar: Android no deja que la app lo impida. Lo no guardado se
  recupera al volver a abrir; para que pregunte, usa el botón Atrás o la × de la pestaña.
- El panel Imprimir no lista impresoras: la impresora, las copias y la doble cara se eligen en el
  diálogo de Android.
- Leer en voz alta no tiene voz en Android.
