-- marcas, tabla de 
-- abreviacion MAR

CREATE TABLE IF NOT EXISTS public.marcas
(
    id bigint NOT NULL,
    nombre TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    CONSTRAINT marcas_pkey PRIMARY KEY (id)
);

ALTER TABLE public.marcas OWNER TO erp;
CREATE SEQUENCE public.marcas_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER TABLE public.marcas_id_seq OWNER TO erp;
ALTER SEQUENCE public.marcas_id_seq OWNED BY public.marcas.id;
ALTER TABLE ONLY public.marcas ALTER COLUMN id SET DEFAULT nextval('public.marcas_id_seq'::regclass);

-- datos
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (278, 'Aconcagua', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (279, 'Act II', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (280, 'Adams', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (281, 'Africana', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (282, 'Sin Marca', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (283, 'Aguai', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (284, 'Ajinomen', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (285, 'Ajinomoto', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (286, 'Alcofen', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (287, 'Alikal', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (288, 'Alka-Seltzer', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (289, 'Amazonas', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (290, 'Ambrosoli', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (291, 'Anapqui', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (292, 'Animalitos', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (293, 'Aquarius', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (294, 'Arcadia', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (295, 'ArcoIris', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (296, 'ARCOR', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (297, 'Aroma', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (298, 'Asatex', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (299, 'Aval', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (300, 'Axe', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (301, 'Baby Ñoño', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (302, 'Babysec', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (303, 'Ballerina', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (304, 'Bayer', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (305, 'Belen', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (306, 'Bolivar', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (307, 'BomBomBum', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (308, 'Bombril', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (309, 'bon o bon', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (310, 'Breik', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (311, 'BRIO', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (312, 'Caiman', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (313, 'Cajamar', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (314, 'Calesita', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (315, 'Calmadol', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (316, 'Calmadolcito', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (317, 'Canel', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (318, 'Caranavi', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (319, 'Carozzi', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (320, 'Casa Real', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (323, 'Caserita', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (324, 'Cat Chow', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (325, 'Cerelac', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (326, 'CERENAT', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (327, 'Ceylan', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (328, 'Chawei', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (329, 'Cheff Gusto', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (330, 'Chips Ahoy', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (331, 'Chiriguano', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (332, 'Chocapic', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (333, 'Chocman', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (334, 'Chocolike', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (335, 'Cif', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (336, 'Coca Cola', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (337, 'Cocky', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (338, 'Cofler', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (340, 'Colgate', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (341, 'Colonia Japonesa', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (342, 'Condor', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (343, 'Confort', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (344, 'Copacabana', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (345, 'Costa', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (346, 'Crick', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (347, 'Criollo', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (348, 'crismelos', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (349, 'Crisol', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (350, 'CureBand', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (351, 'Del Campo', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (352, 'Del Valle', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (354, 'Derby', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (355, 'Digestan', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (356, 'Docile', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (357, 'Don Maximo', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (358, 'Don Vittorio', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (359, 'Donofrio', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (360, 'Doña Gusta', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (361, 'Doraditas', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (362, 'Dove', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (363, 'Downy', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (364, 'Duryea', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (365, 'Ecco', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (366, 'El Ceibo', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (367, 'El Chasqui', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (368, 'El Faro', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (369, 'Elite', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (370, 'Eno', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (371, 'ENTEL', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (372, 'Estracto', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (373, 'Excelsior', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (374, 'Faber Castell', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (376, 'Fanta Naranja', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (377, 'Fanta Papaya', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (375, 'Fanta', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (378, 'Fernando', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (379, 'Field', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (380, 'Fiesta', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (381, 'FINO', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (382, 'Fixer', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (383, 'Flavicold', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (384, 'Fleischmann', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (385, 'Fresh Runy', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (386, 'Fruna', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (388, 'Frutigel', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (389, 'Galleticones', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (353, 'Delizia', '2021-03-21 16:32:08.690371', '2021-10-03 17:46:01.61585');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (387, 'Frush', '2021-03-21 16:32:08.690371', '2021-10-03 17:46:01.621795');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (390, 'Garoto', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (391, 'Gatty', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (392, 'Gauchitas', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (393, 'Gloria', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (394, 'Gomes da Costa', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (395, 'Grosso', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (396, 'Guabira', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (397, 'GUSTOSSI', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (398, 'Harry el Limonero', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (399, 'Head & Shoulders', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (400, 'HP', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (401, 'Huawei', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (403, 'INCO', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (404, 'Iris', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (405, 'Jhonnie Walker', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (406, 'Karina', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (407, 'Kevin', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (408, 'King', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (409, 'Kingston', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (410, 'Klin', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (411, 'Knorr', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (412, 'Kohlberg', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (413, 'Korr', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (414, 'Kotex', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (415, 'Kream', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (416, 'KRIS', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (417, 'L.Ch', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (418, 'L&M', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (419, 'La Belgica', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (420, 'La Estrella', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (421, 'La Francesa', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (422, 'La Suprema', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (423, 'Lacteosbol', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (424, 'Lazzaroni', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (425, 'LGQ', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (427, 'Lidita', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (428, 'Limppano', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (429, 'Lix', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (430, 'Lombarda', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (431, 'Lux', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (432, 'Mabel''s', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (433, 'Maggi', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (434, 'Maira', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (435, 'Maltin', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (436, 'Manu', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (437, 'Maria dorita', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (438, 'Maribel', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (439, 'Maruchan', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (440, 'Marutex', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (441, 'Maver', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (442, 'MegaUlix', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (443, 'Mentisan', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (445, 'Merletto', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (446, 'Misioneros', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (447, 'Miura', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (448, 'Mogul', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (449, 'Monaco', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (450, 'Monami', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (451, 'Mundial', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (452, 'Nabisco', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (453, 'Nescafe', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (454, 'Nestle', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (455, 'Nikolo', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (456, 'Nivea', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (457, 'Nonmedical', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (459, 'Nucita', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (460, 'OLA', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (461, 'Olho', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (462, 'OMO', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (463, 'oreo', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (464, 'Orieta', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (465, 'ORO', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (466, 'Paceña', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (467, 'Palmolive', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (468, 'Paris', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (469, 'Patito', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (470, 'Pegafan', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (471, 'Pepsi', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (472, 'Pepsodent', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (474, 'Phelix', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (475, 'Philips', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (476, 'Phowermax', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (477, 'PIL', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (478, 'Pilfrut', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (479, 'Piposal', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (480, 'Piter', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (402, 'Huggies', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (481, 'Plus', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (482, 'Plusbelle', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (483, 'Podium', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (485, 'Pond''s', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (486, 'Powerade', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (487, 'PRAN', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (488, 'Princesa', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (489, 'Princo', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (490, 'Protex', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (491, 'Pura Vida', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (492, 'Q''Rico', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (493, 'Rayovac', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (495, 'Regia', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (496, 'Resfriol', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (497, 'Rexona', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (498, 'Reyna', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (499, 'Ricas', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (500, 'Rossi', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (501, 'Ruby''s', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (502, 'S.Riicc', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (503, 'Sabonis', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (504, 'Sabrosa', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (505, 'Safra', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (506, 'Salvietti', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (484, 'Pomelo Neus', '2021-03-21 16:32:08.690371', '2021-10-01 19:43:44.80539');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (507, 'Samsung', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (508, 'San Gabriel', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (509, 'San Lucas', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (510, 'San Luis', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (511, 'SAO', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (512, 'Scott', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (513, 'Sedal', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (514, 'Shoogy Boom', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (515, 'Sibarita', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (516, 'Siglo de Oro', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (518, 'Simba Piña', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (519, 'Simba Pomelo', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (520, 'Skip', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (521, 'Sofia', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (522, 'Solamigo', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (523, 'Soy', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (524, 'Sprite', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (525, 'Stege', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (526, 'Sublime', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (528, 'Super Glue', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (530, 'Sylvania', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (532, 'Tapsin', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (533, 'Tasty', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (534, 'Terruño', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (535, 'TIGER', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (536, 'TIGO', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (537, 'Toddy', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (538, 'Torito', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (541, 'Trigal', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (542, 'Tron', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (543, 'Tropical', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (544, 'UHU', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (546, 'Van Camp''s', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (549, 'Vital', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (550, 'VIVA', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (552, 'Wafs', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (553, 'Wanabym', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (554, 'Windsor', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (555, 'X5', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (556, 'Zel', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (557, 'Energy Saving', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (458, 'Nosotras', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (529, 'Surf', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (558, 'RLED', '2021-03-30 22:39:55.071901', '2021-03-30 22:39:55.071901');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (559, 'Refresco Kris', '2021-04-14 15:16:48.600464', '2021-04-14 15:16:48.600464');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (560, 'Led bulb', '2021-04-15 00:55:16.237942', '2021-04-15 00:55:16.237942');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (545, 'Uno', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (561, 'Fiat Lux', '2021-04-16 01:14:13.355272', '2021-04-16 01:14:13.355272');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (564, 'El manjar', '2021-04-19 20:12:08.902066', '2021-04-19 20:12:08.902066');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (565, 'Yone', '2021-09-13 22:29:55.579913', '2021-09-13 22:29:55.579913');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (571, 'Frugelé', '2021-09-16 23:50:07.533048', '2021-09-16 23:50:07.533056');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (277, '7up', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (572, 'TIO LED', '2021-09-17 14:32:54.931684', '2021-09-17 14:32:54.931701');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (573, 'Madison', '2021-09-17 17:49:11.759232', '2021-09-17 17:49:11.759238');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (527, 'Super', '2021-03-21 16:32:08.690371', '2021-09-19 16:08:11.474761');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (473, 'Perlita', '2021-03-21 16:32:08.690371', '2021-09-20 18:32:43.656276');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (539, 'Toscano', '2021-03-21 16:32:08.690371', '2021-09-20 19:14:51.835977');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (575, 'Toskano', '2021-09-20 19:25:27.484262', '2021-09-20 19:25:27.484269');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (494, 'Real', '2021-03-21 16:32:08.690371', '2021-09-23 14:40:28.305359');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (540, 'Tri Malta', '2021-03-21 16:32:08.690371', '2021-09-23 14:40:28.341184');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (563, 'Malta Real', '2021-04-19 16:13:23.684361', '2021-09-23 14:40:28.349031');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (576, 'Sinalco', '2021-09-23 14:40:28.357517', '2021-09-23 14:40:28.357525');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (321, 'Cascadita', '2021-03-21 16:32:08.690371', '2021-10-01 19:43:44.754992');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (322, 'Cascafrut', '2021-03-21 16:32:08.690371', '2021-10-01 19:43:44.789571');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (339, 'Coka Quina', '2021-03-21 16:32:08.690371', '2021-10-01 19:43:44.798272');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (548, 'Villa Santa', '2021-03-21 16:32:08.690371', '2021-10-01 19:43:44.81368');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (577, 'Agua sport', '2021-10-01 19:43:44.820678', '2021-10-01 19:43:44.820684');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (517, 'Simba', '2021-03-21 16:32:08.690371', '2021-03-21 16:32:08.690371');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (531, 'Tampico', '2021-03-21 16:32:08.690371', '2021-10-03 17:46:01.627105');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (547, 'Vaquita', '2021-03-21 16:32:08.690371', '2021-10-03 17:46:01.63225');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (578, 'Ice Fruit', '2021-10-03 17:46:01.637599', '2021-10-03 17:46:01.637609');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (579, 'Lider', '2021-10-03 20:19:45.504814', '2021-10-03 20:19:45.504829');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (580, 'Sin marca', '2021-10-04 18:46:51.945316', '2021-10-04 18:46:51.945331');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (581, 'Skill', '2021-10-04 20:11:03.935183', '2021-10-04 20:11:03.935188');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (582, 'Premier', '2021-10-04 20:25:17.215396', '2021-10-04 20:25:17.215401');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (583, 'Bombillas del sur', '2021-10-05 16:22:26.671003', '2021-10-05 16:22:26.67102');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (584, 'Stabilo', '2021-10-05 18:52:27.49875', '2021-10-05 18:52:27.498808');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (585, 'Gillette', '2021-10-05 19:17:30.223935', '2021-10-05 19:17:30.223942');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (586, 'Winner', '2021-10-05 20:57:08.159518', '2021-10-05 20:57:08.159525');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (587, 'Canadios', '2021-10-05 21:10:05.911344', '2021-10-05 21:10:05.911351');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (589, 'Acrilex', '2021-10-05 21:41:54.571858', '2021-10-05 21:41:54.571863');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (590, 'Plasticolor', '2021-10-05 22:01:04.819848', '2021-10-05 22:01:04.819855');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (591, 'Tio LED', '2021-10-05 22:06:47.566807', '2021-10-05 22:06:47.566812');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (592, 'Playing Cards', '2021-10-05 22:25:52.969306', '2021-10-05 22:26:36.188252');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (593, 'UNU', '2021-10-05 22:26:36.19299', '2021-10-05 22:26:36.192994');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (594, 'JBL', '2021-10-05 23:23:43.142358', '2021-10-05 23:23:43.142382');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (595, 'Carpicola', '2021-10-05 23:55:03.710937', '2021-10-05 23:55:03.710944');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (600, 'Zuixua', '2021-10-06 23:18:25.018282', '2021-10-06 23:18:25.018299');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (599, 'Marko', '2021-10-06 22:53:45.941883', '2021-10-06 22:53:45.941889');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (601, 'Acricolor', '2021-10-11 21:05:58.911623', '2021-10-11 21:05:58.911639');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (602, 'Feverhand Faber', '2021-10-11 22:20:08.19399', '2021-10-11 22:20:08.193998');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (603, 'Full color', '2021-10-11 22:57:13.702802', '2021-10-11 22:57:13.70281');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (596, 'Lechuga', '2021-10-06 00:05:28.85533', '2021-10-06 00:13:23.71019');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (597, 'Chirimoya', '2021-10-06 00:05:28.860683', '2021-10-06 00:13:23.714722');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (598, 'Vaselina', '2021-10-06 00:08:54.81551', '2021-10-06 00:13:23.7192');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (566, 'Exam Boil', '2021-09-13 23:34:16.282821', '2021-10-06 22:53:45.933935');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (604, 'SONY', '2021-10-11 23:48:46.741807', '2021-10-11 23:48:46.741814');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (605, 'Penguin', '2021-10-16 23:33:34.514467', '2021-10-16 23:33:34.514474');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (606, 'Top Line', '2021-10-16 23:58:07.577311', '2021-10-16 23:58:07.577319');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (607, 'Victoria', '2021-10-22 21:49:45.214522', '2021-10-22 21:49:45.214539');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (608, 'The Castell', '2021-10-22 23:18:20.784902', '2021-10-22 23:18:20.784914');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (609, 'La Europea', '2021-10-23 23:25:28.527276', '2021-10-23 23:25:28.527288');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (610, 'Lentelux', '2021-10-25 13:50:38.501308', '2021-10-25 13:50:38.501337');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (611, 'Crysanthemum', '2021-10-25 14:33:48.224561', '2021-10-25 14:33:48.224578');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (612, 'Great Hall', '2021-10-25 14:48:08.714319', '2021-10-25 14:48:08.714327');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (613, 'Pelikan', '2021-10-25 15:45:12.725831', '2021-10-25 15:45:12.725848');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (614, 'Great Wall', '2021-10-26 22:34:32.375714', '2021-10-26 22:34:32.375732');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (615, 'Laubsageblatter', '2021-10-26 22:42:47.272066', '2021-10-26 22:43:33.446159');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (616, 'ABC', '2021-10-26 23:31:13.177331', '2021-10-26 23:31:13.177335');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (617, 'Merheje', '2021-11-01 01:05:41.372704', '2021-11-01 01:05:41.372721');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (618, 'XZV', '2021-11-01 01:10:30.739957', '2021-11-01 01:10:30.739964');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (619, 'Hair Grips', '2021-11-01 01:24:59.142361', '2021-11-01 01:24:59.14237');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (620, 'Sea Gull', '2021-11-01 01:36:39.833374', '2021-11-01 01:36:39.833382');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (621, 'Theoto', '2021-11-01 01:50:23.354409', '2021-11-01 01:50:23.354414');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (622, 'Five Stick', '2021-11-01 02:11:07.897253', '2021-11-01 02:11:07.89726');
INSERT INTO public.marcas (id, nombre, created_at, updated_at) VALUES (623, 'Milcar', '2021-11-03 23:27:07.533038', '2021-11-03 23:27:07.533052');

-- ajusta numero de ultimo registro
SELECT pg_catalog.setval('public.marcas_id_seq', 623, true);

-- constraints
-- trigger para preparar el insert
CREATE OR REPLACE FUNCTION marcas_insert_prepara() 
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
    BEFORE INSERT ON marcas
    FOR EACH ROW
    EXECUTE FUNCTION marcas_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION marcas_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON marcas
    FOR EACH ROW
    EXECUTE FUNCTION marcas_update_prepara();

