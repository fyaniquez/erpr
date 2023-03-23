--
-- PostgreSQL database dump
--

-- Dumped from database version 13.9 (Debian 13.9-0+deb11u1)
-- Dumped by pg_dump version 13.9 (Debian 13.9-0+deb11u1)

-- Started on 2022-12-26 11:27:52 -04

CREATE TABLE public.inventariados (
    id bigint NOT NULL,
    cantidad integer NOT NULL,
    vencimiento timestamp without time zone,
    producto_id bigint NOT NULL,
    inventario_id bigint NOT NULL,
    created_at timestamp(6) without time zone NOT NULL,
    updated_at timestamp(6) without time zone NOT NULL
);


ALTER TABLE public.inventariados OWNER TO erp;

--
-- TOC entry 239 (class 1259 OID 16534)
-- Name: inventariados_id_seq; Type: SEQUENCE; Schema: public; Owner: erp
--

CREATE SEQUENCE public.inventariados_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.inventariados_id_seq OWNER TO erp;

--
-- TOC entry 3245 (class 0 OID 0)
-- Dependencies: 239
-- Name: inventariados_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: erp
--

ALTER SEQUENCE public.inventariados_id_seq OWNED BY public.inventariados.id;


--
-- TOC entry 3099 (class 2604 OID 16681)
-- Name: inventariados id; Type: DEFAULT; Schema: public; Owner: erp
--

ALTER TABLE ONLY public.inventariados ALTER COLUMN id SET DEFAULT nextval('public.inventariados_id_seq'::regclass);


-- data

insert into inventariados (id, cantidad, vencimiento, producto_id, inventario_id, created_at, updated_at) values (1,200,'2021-10-01 22:39:03', 1514,1,'2021-10-01 22:39:03.686608','2021-10-01 22:39:03.686609');

SELECT pg_catalog.setval('public.inventariados_id_seq', 2, true);


--
-- TOC entry 3104 (class 2606 OID 16735)
-- Name: inventariados inventariados_pkey; Type: CONSTRAINT; Schema: public; Owner: erp
--

ALTER TABLE ONLY public.inventariados
    ADD CONSTRAINT inventariados_pkey PRIMARY KEY (id);


--
-- TOC entry 3100 (class 1259 OID 16790)
-- Name: index_inventariados_on_empleado_id; Type: INDEX; Schema: public; Owner: erp
--

CREATE INDEX index_inventariados_on_inventario_id ON public.inventariados USING btree (inventario_id);


CREATE INDEX index_inventariados_on_productos ON public.inventariados USING btree (producto_id);


ALTER TABLE ONLY public.inventariados
    ADD CONSTRAINT fk_inventariados_productos FOREIGN KEY (producto_id) REFERENCES public.productos(id) ON UPDATE RESTRICT ON DELETE RESTRICT;


ALTER TABLE ONLY public.inventariados
    ADD CONSTRAINT fk_rails_6c643ccb9a FOREIGN KEY (inventario_id) REFERENCES public.inventarios(id);



-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION inventariados_insert_prepara() 
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
    BEFORE INSERT ON inventariados
    FOR EACH ROW
    EXECUTE FUNCTION inventariados_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION inventariados_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON inventariados
    FOR EACH ROW
    EXECUTE FUNCTION inventariados_update_prepara();
