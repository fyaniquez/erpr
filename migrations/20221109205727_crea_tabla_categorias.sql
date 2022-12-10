-- categorias clasifica productos por grupos
CREATE TABLE IF NOT EXISTS public.categorias
(
    id BIGINT NOT NULL,
    nombre TEXT NOT NULL,
    capitulo_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    CONSTRAINT categorias_pkey PRIMARY KEY (id)
);

ALTER TABLE public.categorias OWNER TO erp;
CREATE SEQUENCE public.categorias_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER TABLE public.categorias_id_seq OWNER TO erp;
ALTER SEQUENCE public.categorias_id_seq OWNED BY public.categorias.id;
ALTER TABLE ONLY public.categorias ALTER COLUMN id SET DEFAULT nextval('public.categorias_id_seq'::regclass);
SELECT pg_catalog.setval('public.categorias_id_seq', 606, true);

-- datos
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (297, 'Café', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (298, 'Caldo', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (299, 'Champiñones', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (300, 'Choclo', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (301, 'Cigarro', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (302, 'Concentrado', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (303, 'Condimento', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (304, 'Crema polvo', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (305, 'Duraznos', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (306, 'Escabeche', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (307, 'Esencia', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (308, 'Fideo', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (309, 'Fideo precocido', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (310, 'Flan', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (311, 'Frangollo', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (313, 'Harina', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (314, 'Helado', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (315, 'Huevo', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (316, 'Jurel', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (317, 'Lenteja', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (318, 'Levadura', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (319, 'Maicena', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (320, 'Maíz', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (321, 'Maní', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (322, 'Manteca', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (323, 'Mantequilla', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (324, 'Margarina', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (325, 'Mermelada', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (326, 'Miel', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (327, 'Pastas', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (328, 'Picadillo', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (329, 'Pipocas microondas', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (330, 'Polvo Hornear', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (331, 'Postres', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (332, 'Pudín', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (333, 'Puré papas', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (334, 'Puré tomate', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (335, 'Quinua', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (336, 'Salsa', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (337, 'Sardina', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (338, 'Sazonador', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (339, 'Sémola', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (340, 'Servilleta', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (341, 'Sopa polvo', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (342, 'Té', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (343, 'Trigo', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (345, 'Adhesivo', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (346, 'Aguja', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (347, 'Bolsas', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (348, 'Bolsas de basura', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (349, 'CD', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (350, 'Cereal', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (352, 'Cocoa', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (353, 'Cortauñas', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (354, 'Crema de zapatos', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (356, 'DVD', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (358, 'Escobilla', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (359, 'Esponja', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (361, 'Fósforos', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (363, 'Globo', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (364, 'Guantes', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (366, 'Linternas', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (367, 'Pila', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (368, 'Pinza', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (369, 'Piñatas', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (370, 'Tongos', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (371, 'Vela', 2, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (373, 'Bebida Láctea', 3, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (374, 'Bebida polvo', 3, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (375, 'Cerveza Negra', 3, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (376, 'Energizante', 3, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (378, 'Infantil', 3, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (379, 'Isotónica', 3, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (381, 'Soja', 3, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (382, 'Zero', 3, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (383, 'Chorizo', 4, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (384, 'Chuletas', 4, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (385, 'Hamburguesa', 4, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (386, 'Jamón', 4, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (387, 'Milanesas', 4, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (388, 'Nuggets', 4, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (389, 'Paté', 4, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (390, 'Puerco', 4, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (391, 'Pollo', 4, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (392, 'Queso de chancho', 4, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (393, 'Sillpancho', 4, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (394, 'Chip', 5, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (395, 'Llamadas', 5, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (396, 'Recarga', 5, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (397, 'Tarjeta', 5, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (398, 'Acondicionador', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (399, 'Bloqueador Solar', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (377, 'Gaseosa', 3, '2021-03-21 17:40:22.82341', '2021-09-23 14:40:50.061813');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (372, 'Agua', 3, '2021-03-21 17:40:22.82341', '2021-10-01 19:48:26.044223');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (380, 'Jugo', 3, '2021-03-21 17:40:22.82341', '2021-10-03 17:49:39.03617');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (365, 'Horquillas', 2, '2021-03-21 17:40:22.82341', '2021-11-01 01:26:19.005052');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (351, 'Chinches', 2, '2021-03-21 17:40:22.82341', '2021-10-16 23:34:20.362091');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (357, 'Encendedor', 2, '2021-03-21 17:40:22.82341', '2021-10-23 23:21:16.543113');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (360, 'Focos', 2, '2021-03-21 17:40:22.82341', '2021-10-25 13:51:53.584036');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (355, 'Cubiertos', 2, '2021-03-21 17:40:22.82341', '2021-10-23 23:28:30.755738');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (362, 'Ganchos', 2, '2021-03-21 17:40:22.82341', '2021-11-01 01:37:24.375391');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (400, 'Colonia', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (401, 'Crema Corporal', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (402, 'Crema de Manos', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (403, 'Crema de Peinar', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (404, 'Crema Faciales', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (405, 'Depilación', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (406, 'Desmaquillante', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (407, 'Desodorante', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (408, 'Fijador', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (409, 'Gel', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (410, 'Loción de Afeitado', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (411, 'Máquina de afeitar', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (412, 'Perfumes', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (413, 'Tinte', 6, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (418, 'Flash Disk', 7, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (419, 'Memoria', 7, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (420, 'Carnes frias', 8, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (421, 'Mortadela', 8, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (422, 'Salchichas', 8, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (423, 'Juguete', 9, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (424, 'Videos', 9, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (425, 'Cinta aislante', 10, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (427, 'Teflon', 10, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (428, 'Limón', 11, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (429, 'Alfajor', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (430, 'Barra', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (431, 'Bombon', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (432, 'Caramelo', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (433, 'Chicle', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (434, 'Chocolate', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (435, 'Chupete', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (436, 'Gomitas', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (437, 'Grageas', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (438, 'MarshMallows', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (439, 'Masticables', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (440, 'Pastillas', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (441, 'Turrón', 12, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (442, 'Alcohol', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (443, 'Alcohol en gel', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (444, 'Cepillo', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (445, 'Dentrífico', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (446, 'Enjuague bucal', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (447, 'Jabón Líquido', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (448, 'Jaboncillo', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (449, 'Loción', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (450, 'Pañales', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (451, 'Pañuelos', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (453, 'Seda Dental', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (454, 'Shampoo', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (455, 'Talco', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (456, 'Toalla Papel', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (457, 'Toallitas Húmedas', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (458, 'Alfajor', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (459, 'Bizcocho', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (460, 'Budin', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (461, 'Empanada', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (462, 'Galleta agua', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (463, 'Galleta dulce', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (464, 'Galleta integral', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (465, 'Galleta salada', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (466, 'Galleta wafer', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (467, 'Masas', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (468, 'Pan', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (469, 'Pie', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (470, 'Queque', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (471, 'Rollos', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (472, 'Tortas', 15, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (473, 'Crema de Leche', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (474, 'Dulce de leche', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (475, 'Leche', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (476, 'Leche Condensada', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (477, 'Leche de soya', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (478, 'Leche Descremada', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (479, 'Leche Deslactosada', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (480, 'Leche Polvo', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (481, 'Leche Evaporada', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (482, 'Leche infantil', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (483, 'Leche Saborizada', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (484, 'Manjar', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (485, 'Otros', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (286, 'Aceite', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (287, 'Aceitunas', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (288, 'Aji Amarillo', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (289, 'Aji Colorado', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (290, 'Anchoveta', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (291, 'Api', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (292, 'Arroz', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (293, 'Arveja', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (294, 'Atun', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (295, 'Avena', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (296, 'Azúcar', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (486, 'Queso', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (487, 'Yoghurt', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (488, 'Yogourt infantil', 16, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (490, 'CD', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (492, 'Cuaderno', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (493, 'Cutter', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (494, 'Estuche Geo.', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (495, 'Fastener', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (497, 'Folder', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (500, 'Hojas Carpeta', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (501, 'Láminas', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (504, 'Papel Bond', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (505, 'Papel regalo', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (506, 'Papel resma', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (507, 'Pegamento', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (511, 'Tijera', 17, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (496, 'Flip', 17, '2021-03-21 17:40:22.82341', '2021-10-04 20:03:09.457662');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (498, 'Goma', 17, '2021-03-21 17:40:22.82341', '2021-10-16 21:37:44.624127');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (499, 'Grampas', 17, '2021-03-21 17:40:22.82341', '2021-10-05 23:33:09.565869');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (415, 'Audífono', 7, '2021-03-21 17:40:22.82341', '2021-10-05 23:24:26.783166');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (417, 'Cargador', 7, '2021-03-21 17:40:22.82341', '2021-10-05 23:29:27.155091');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (416, 'Cable Usb', 7, '2021-03-21 17:40:22.82341', '2021-10-12 00:22:39.721892');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (502, 'Lapíz', 17, '2021-03-21 17:40:22.82341', '2021-10-08 23:09:48.42103');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (510, 'Silicona', 17, '2021-03-21 17:40:22.82341', '2021-10-10 00:02:55.463269');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (503, 'Marcador', 17, '2021-03-21 17:40:22.82341', '2021-10-11 22:20:32.253436');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (491, 'Compas', 17, '2021-03-21 17:40:22.82341', '2021-10-16 23:20:00.711131');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (426, 'Cinta Masking', 10, '2021-03-21 17:40:22.82341', '2021-11-01 02:11:29.499449');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (508, 'Regla', 17, '2021-03-21 17:40:22.82341', '2021-10-17 00:01:37.466216');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (509, 'Scotch', 17, '2021-03-21 17:40:22.82341', '2021-11-04 14:23:53.596463');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (512, 'Cerveza', 18, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (513, 'Cuba Libre', 18, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (514, 'Lix', 18, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (515, 'Ron', 18, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (516, 'Sidra', 18, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (517, 'Singani', 18, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (518, 'Varadero', 18, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (519, 'Vino', 18, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (520, 'Whisky', 18, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (521, 'Ambientador', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (522, 'Aromatizante', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (523, 'Cera', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (524, 'Cloro', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (525, 'Cocina', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (526, 'Desinfectante', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (527, 'Detergente', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (528, 'Escoba', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (529, 'Insecticida', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (530, 'Jabon', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (531, 'Lavandina', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (532, 'Limpiador Baño', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (533, 'Limpiador Cocina', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (534, 'Limpiador Muebles', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (535, 'Limpiador Pisos', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (536, 'Limpiador Vajilla', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (537, 'Limpiador Vidrios', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (538, 'Muebles', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (539, 'Papeles', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (540, 'Pisos', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (541, 'Suavizante', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (542, 'Virutilla', 19, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (543, 'Alimento p/Gato', 20, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (544, 'Alimento p/Perro', 20, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (545, 'Arena p/Gato', 20, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (546, 'Paneton', 21, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (547, 'Algodón', 22, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (548, 'Analgésico', 22, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (549, 'Antiácido', 22, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (550, 'Antigripal', 22, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (551, 'Aspirina', 22, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (552, 'Barbijo', 22, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (553, 'Curita', 22, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (554, 'Digestivo', 22, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (555, 'Preservativo', 22, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (556, 'Ungüento', 22, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (557, 'Vitamina', 22, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (558, 'Fotocopias', 23, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (559, 'Impresiones', 23, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (560, 'Pagos', 23, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (561, 'Transcripción', 23, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (562, 'Wi-Fi', 23, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (563, 'Chicharron', 24, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (564, 'Chisito', 24, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (565, 'Palitos', 24, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (566, 'Papas Fritas', 24, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (567, 'Pipocas', 24, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (568, 'Semillas', 24, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (569, 'Tortillas', 24, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (414, 'Toallas Femeninas', 13, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (570, 'Chocolate', 1, '2021-04-16 01:27:11.277182', '2021-04-16 01:27:11.277182');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (312, 'Gelatina', 1, '2021-03-21 17:40:22.82341', '2021-03-21 17:40:22.82341');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (571, 'Cartulina', 17, '2021-08-30 17:31:10.45966', '2021-08-30 17:31:10.459663');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (452, 'Papel Higiénico', 13, '2021-03-21 17:40:22.82341', '2021-09-20 18:20:52.468969');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (344, 'Vinagre', 1, '2021-03-21 17:40:22.82341', '2021-09-20 19:25:44.803305');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (573, 'folder', 17, '2021-10-04 02:42:17.306407', '2021-10-04 02:47:25.8393');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (574, 'Funda', 17, '2021-10-04 20:11:28.619593', '2021-10-04 20:11:28.619595');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (572, 'Hojas', 17, '2021-10-03 20:21:19.349293', '2021-10-04 20:27:47.159585');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (575, 'Bombillas', 2, '2021-10-05 16:26:05.7307', '2021-10-05 16:26:05.730702');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (576, 'Sobres', 17, '2021-10-05 18:28:36.484493', '2021-10-05 18:28:36.484496');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (577, 'Bolsas', 17, '2021-10-05 18:43:55.395278', '2021-10-05 18:43:55.395281');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (579, 'Afeitadora', 13, '2021-10-05 19:18:34.440409', '2021-10-05 19:18:34.440412');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (581, 'platos', 2, '2021-10-05 21:06:42.293855', '2021-10-05 21:06:42.293858');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (583, 'Velas', 2, '2021-10-05 21:16:16.157632', '2021-10-05 21:16:16.157635');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (584, 'Gel fijador', 13, '2021-10-05 21:36:49.182646', '2021-10-05 21:36:49.182648');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (585, 'Tinta p/tela', 17, '2021-10-05 21:42:28.027333', '2021-10-05 21:42:28.027335');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (587, 'Dados', 9, '2021-10-05 22:17:03.840937', '2021-10-05 22:17:03.840939');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (588, 'Casinos', 9, '2021-10-05 22:28:12.377175', '2021-10-05 22:28:51.025755');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (582, 'Adornos', 17, '2021-10-05 21:10:37.515486', '2021-10-05 23:49:00.323795');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (489, 'Bolígrafo', 17, '2021-03-21 17:40:22.82341', '2021-10-06 23:19:39.772663');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (590, 'papel lustroso', 17, '2021-10-08 23:44:11.404566', '2021-10-08 23:44:11.404569');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (592, 'Engrampadora', 17, '2021-10-09 23:56:06.443276', '2021-10-09 23:56:06.44328');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (591, 'silicona', 17, '2021-10-09 23:33:40.464982', '2021-10-10 00:03:56.660942');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (593, 'Portaminas', 17, '2021-10-10 00:11:23.747253', '2021-10-10 00:11:23.747256');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (589, 'Micropunta', 17, '2021-10-05 23:41:13.315315', '2021-10-11 20:06:38.808252');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (594, 'Corrector', 17, '2021-10-11 21:06:20.806885', '2021-10-11 21:06:20.806888');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (595, 'Tinta p/impresora', 17, '2021-10-11 22:57:53.822489', '2021-10-11 22:57:53.822491');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (604, 'Sierras', 10, '2021-10-26 22:44:08.004426', '2021-10-26 22:44:08.00443');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (596, 'Memoria microSD', 7, '2021-10-11 23:44:11.342269', '2021-10-12 00:02:50.563918');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (597, 'Papel crepé', 17, '2021-10-16 23:46:54.530709', '2021-10-16 23:46:54.530712');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (598, 'Ligas', 17, '2021-10-22 21:50:18.832599', '2021-10-22 21:50:18.832602');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (599, 'Cuadernillo', 17, '2021-10-22 23:19:09.141167', '2021-10-22 23:19:09.141171');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (600, 'Platillos', 2, '2021-10-24 16:16:53.9019', '2021-10-24 16:16:53.901902');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (601, 'Alfileres', 2, '2021-10-25 14:34:33.61919', '2021-10-25 14:34:33.619193');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (602, 'Grampas', 2, '2021-10-25 14:49:09.918564', '2021-10-25 14:49:09.918567');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (586, 'Plastilina', 17, '2021-10-05 22:01:25.792804', '2021-10-25 15:45:34.893595');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (580, 'Cuadernos', 17, '2021-10-05 20:57:36.807851', '2021-10-26 23:32:17.560044');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (603, 'Tajadores', 17, '2021-10-26 22:34:51.374767', '2021-10-26 23:40:52.537761');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (605, 'Pinzas', 2, '2021-11-01 01:06:14.403797', '2021-11-01 01:10:48.137716');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (606, 'Mondadientes', 2, '2021-11-01 01:51:33.021498', '2021-11-01 01:51:33.0215');
INSERT INTO public.categorias (id, nombre, capitulo_id, created_at, updated_at) VALUES (578, 'Resaltadores', 17, '2021-10-05 18:53:13.354942', '2021-11-03 23:27:29.600636');

-- indices
CREATE INDEX index_categorias_on_capitulo_id ON public.categorias USING btree (capitulo_id);

-- constraints
ALTER TABLE ONLY public.categorias
    ADD CONSTRAINT fk_cat_cap FOREIGN KEY (capitulo_id) REFERENCES public.capitulos(id);

ALTER TABLE public.categorias
    ADD CONSTRAINT uk_cap_id_nombre UNIQUE (capitulo_id, nombre);

-- trigger para preparar el insert
CREATE OR REPLACE FUNCTION categorias_insert_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.created_at = now();
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER insert_prepara
    BEFORE INSERT ON categorias
    FOR EACH ROW
    EXECUTE FUNCTION categorias_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION categorias_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON categorias
    FOR EACH ROW
    EXECUTE FUNCTION categorias_update_prepara();

