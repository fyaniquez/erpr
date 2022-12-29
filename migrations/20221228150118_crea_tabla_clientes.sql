--

CREATE TABLE public.clientes (
    id bigint NOT NULL,
    nombre text NOT NULL,
    documento text NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


ALTER TABLE public.clientes OWNER TO erp;

--
-- TOC entry 3249 (class 0 OID 0)
-- Dependencies: 281
-- Name: TABLE clientes; Type: COMMENT; Schema: public; Owner: erp
--

COMMENT ON TABLE public.clientes IS 'registro de clientes naturales y juridicas que interactuan con el sistema como usuarios, cajeros, clientes, proveedores, etc.';


--
-- TOC entry 282 (class 1259 OID 17334)
-- Name: clientes_id_seq; Type: SEQUENCE; Schema: public; Owner: erp
--

CREATE SEQUENCE public.clientes_id_seq
    AS bigint
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.clientes_id_seq OWNER TO erp;

--
-- TOC entry 3250 (class 0 OID 0)
-- Dependencies: 282
-- Name: clientes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: erp
--

ALTER SEQUENCE public.clientes_id_seq OWNED BY public.clientes.id;


--
-- TOC entry 3105 (class 2604 OID 17336)
-- Name: clientes id; Type: DEFAULT; Schema: public; Owner: erp
--

ALTER TABLE ONLY public.clientes ALTER COLUMN id SET DEFAULT nextval('public.clientes_id_seq'::regclass);


--
-- TOC entry 3242 (class 0 OID 17331)
-- Dependencies: 281
-- Data for Name: clientes; Type: TABLE DATA; Schema: public; Owner: erp
--

INSERT INTO public.clientes (id, nombre, documento, created_at, updated_at) VALUES (1, 'Yañiquez Monasterios, Favio Jorge', '2710988016', '2022-12-22 15:00:00', '2022-12-22 15:00:00');
INSERT INTO public.clientes (id, nombre, documento, created_at, updated_at) VALUES (2, 'Monasterios Nina, Maria Luisa', '2228050', '2022-12-22 15:00:00', '2022-12-22 15:00:00');


--
-- TOC entry 3251 (class 0 OID 0)
-- Dependencies: 282
-- Name: clientes_id_seq; Type: SEQUENCE SET; Schema: public; Owner: erp
--

SELECT pg_catalog.setval('public.clientes_id_seq', 2, true);


--
-- TOC entry 3107 (class 2606 OID 17344)
-- Name: clientes clientes_pkey; Type: CONSTRAINT; Schema: public; Owner: erp
--

ALTER TABLE ONLY public.clientes
    ADD CONSTRAINT clientes_pkey PRIMARY KEY (id);


--
-- TOC entry 3109 (class 2606 OID 17348)
-- Name: clientes uk_per_documento; Type: CONSTRAINT; Schema: public; Owner: erp
--

ALTER TABLE ONLY public.clientes
    ADD CONSTRAINT uk_per_documento UNIQUE (documento);


--
-- TOC entry 3111 (class 2606 OID 17346)
-- Name: clientes uk_per_nombre; Type: CONSTRAINT; Schema: public; Owner: erp
--

ALTER TABLE ONLY public.clientes
    ADD CONSTRAINT uk_per_nombre UNIQUE (nombre);


-- Completed on 2022-12-28 15:01:24 -04

--
-- PostgreSQL database dump complete
--

-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION clientes_insert_prepara() 
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
    BEFORE INSERT ON clientes
    FOR EACH ROW
    EXECUTE FUNCTION clientes_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION clientes_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON clientes
    FOR EACH ROW
    EXECUTE FUNCTION clientes_update_prepara();
