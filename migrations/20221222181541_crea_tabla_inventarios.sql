--
CREATE TABLE public.inventarios (
    id bigint NOT NULL,
    nombre TEXT NOT NULL,
    sucursal_id BIGINT NOT NULL,
    fecha timestamp without time zone NOT NULL,
    estado TEXT NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


ALTER TABLE public.inventarios OWNER TO erp;

COMMENT ON COLUMN public.inventarios.estado IS 'estado actual dentro del ciclo de vida de inventario creando-activo-inactivo';
COMMENT ON COLUMN public.inventarios.fecha IS 'fecha desde la que es activo el inventario';

CREATE SEQUENCE public.inventarios_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.inventarios_id_seq OWNER TO erp;

--
-- TOC entry 3241 (class 0 OID 0)
-- Dependencies: 241
-- Name: inventarios_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: erp
--

ALTER SEQUENCE public.inventarios_id_seq OWNED BY public.inventarios.id;


--
-- TOC entry 3099 (class 2604 OID 16682)
-- Name: inventarios id; Type: DEFAULT; Schema: public; Owner: erp
--

ALTER TABLE ONLY public.inventarios ALTER COLUMN id SET DEFAULT nextval('public.inventarios_id_seq'::regclass);


--
-- TOC entry 3233 (class 0 OID 16536)
-- Dependencies: 240
-- Data for Name: inventarios; Type: TABLE DATA; Schema: public; Owner: erp
--

INSERT INTO public.inventarios (id, nombre, sucursal_id, fecha, estado, created_at, updated_at) VALUES (1, 'inventario 2021', 1, '2021-03-31 23:04:00', 'inactivo', '2021-03-21 23:05:17.034993', '2021-03-21 23:05:17.034993');
INSERT INTO public.inventarios (id, nombre, sucursal_id, fecha, estado, created_at, updated_at) VALUES (2, 'inventario 2022', 1, '2022-03-31 23:04:00', 'activo', '2022-03-21 23:05:17.034993', '2022-03-21 23:05:17.034993');


--
-- TOC entry 3242 (class 0 OID 0)
-- Dependencies: 241
-- Name: inventarios_id_seq; Type: SEQUENCE SET; Schema: public; Owner: erp
--

SELECT pg_catalog.setval('public.inventarios_id_seq', 2, true);


--
-- TOC entry 3101 (class 2606 OID 16737)
-- Name: inventarios inventarios_pkey; Type: CONSTRAINT; Schema: public; Owner: erp
--

ALTER TABLE ONLY public.inventarios
    ADD CONSTRAINT inventarios_pkey PRIMARY KEY (id);

--
-- TOC entry 3102 (class 2606 OID 17131)
-- Name: inventarios fk_inventario_puesto_usuario; Type: FK CONSTRAINT; Schema: public; Owner: erp
--

CREATE INDEX index_inventarios_on_sucursal_id 
    ON public.inventarios USING btree (sucursal_id);
ALTER TABLE ONLY public.inventarios
    ADD CONSTRAINT fk_inv_suc FOREIGN KEY (sucursal_id) 
    REFERENCES public.sucursales(id) ON UPDATE RESTRICT ON DELETE RESTRICT;

ALTER TABLE ONLY public.inventarios
    ADD CONSTRAINT uk_inv_sucursal_nombre UNIQUE (sucursal_id, nombre);

-- Completed on 2022-12-22 18:16:00 -04

--
-- PostgreSQL database dump complete
--

-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION inventarios_insert_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
    NEW.fecha = now();
    NEW.estado = 'Inactivo';
	NEW.created_at = now();
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER insert_prepara
    BEFORE INSERT ON inventarios
    FOR EACH ROW
    EXECUTE FUNCTION inventarios_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION inventarios_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON inventarios
    FOR EACH ROW
    EXECUTE FUNCTION inventarios_update_prepara();
