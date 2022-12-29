--
-- PostgreSQL database dump
--

-- Dumped from database version 13.9 (Debian 13.9-0+deb11u1)
-- Dumped by pg_dump version 13.9 (Debian 13.9-0+deb11u1)

-- Started on 2022-12-26 11:27:52 -04

CREATE TABLE public.inventariados (
    id bigint NOT NULL,
    cantidad integer NOT NULL,
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

insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (5,200,1514,1,'2021-10-01 22:39:03.686608','2021-10-01 22:39:03.686609');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (3,400,1221,1,'2021-08-30 14:53:22.298627','2021-08-30 14:53:22.298628');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (2,1000,1510,1,'2021-05-18 01:58:32.954815','2021-05-18 01:58:32.954815');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (4,600,1516,1,'2021-10-01 22:34:25.575083','2021-10-01 22:34:25.575085');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (7,100,1512,1,'2021-10-01 23:41:27.060647','2021-10-01 23:41:27.060648');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (8,100,1517,1,'2021-10-02 00:16:19.191956','2021-10-02 00:16:19.191957');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (9,200,1442,1,'2021-10-02 00:18:08.347705','2021-10-02 00:18:08.347707');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (10,200,1444,1,'2021-10-02 00:18:25.081257','2021-10-02 00:18:25.081259');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (11,100,848,1,'2021-10-02 00:20:31.094146','2021-10-02 00:20:31.094148');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (12,600,743,1,'2021-10-03 02:46:42.55937','2021-10-03 02:46:42.559372');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (13,100,1415,1,'2021-10-03 16:07:24.285917','2021-10-03 16:07:24.285919');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (14,300,1519,1,'2021-10-03 16:28:58.531055','2021-10-03 16:28:58.531057');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (15,100,1521,1,'2021-10-03 16:31:51.578164','2021-10-03 16:31:51.578165');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (16,400,938,1,'2021-10-03 16:34:54.315208','2021-10-03 16:34:54.315209');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (17,600,1464,1,'2021-10-03 16:35:53.683334','2021-10-03 16:35:53.683336');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (18,600,1522,1,'2021-10-03 16:43:27.575115','2021-10-03 16:43:27.575117');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (19,500,1523,1,'2021-10-03 16:47:01.755884','2021-10-03 16:47:01.755886');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (20,600,826,1,'2021-10-03 16:50:50.333219','2021-10-03 16:50:50.333221');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (21,300,1290,1,'2021-10-03 16:57:07.891718','2021-10-03 16:57:07.89172');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (22,300,873,1,'2021-10-03 17:11:50.616053','2021-10-03 17:11:50.616055');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (23,100,745,1,'2021-10-03 17:13:58.507661','2021-10-03 17:13:58.507666');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (24,200,1524,1,'2021-10-03 17:51:49.319678','2021-10-03 17:51:49.31968');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (25,400,1241,1,'2021-10-03 18:28:52.793733','2021-10-03 18:28:52.793735');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (26,500,746,1,'2021-10-03 18:40:22.220662','2021-10-03 18:40:22.220663');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (27,600,1176,1,'2021-10-03 18:41:00.373045','2021-10-03 18:41:00.373046');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (28,500,747,1,'2021-10-03 18:41:20.388382','2021-10-03 18:41:20.388384');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (29,200,1415,1,'2021-10-03 18:42:10.487717','2021-10-03 18:42:10.487719');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (30,100,1416,1,'2021-10-03 18:42:29.131797','2021-10-03 18:42:29.131799');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (31,300,1399,1,'2021-10-03 18:42:59.388312','2021-10-03 18:42:59.388314');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (32,600,1290,1,'2021-10-03 18:51:17.123409','2021-10-03 18:51:17.123411');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (33,600,1525,1,'2021-10-03 19:01:13.723102','2021-10-03 19:01:13.723104');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (34,900,1526,1,'2021-10-03 19:04:51.167768','2021-10-03 19:04:51.167769');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (35,700,1515,1,'2021-10-03 19:12:26.263746','2021-10-03 19:12:26.263748');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (36,300,1520,1,'2021-10-03 19:17:21.334355','2021-10-03 19:17:21.334356');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (37,200,828,1,'2021-10-03 19:17:54.937347','2021-10-03 19:17:54.937349');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (38,600,1460,1,'2021-10-03 19:18:09.921698','2021-10-03 19:18:09.9217');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (39,600,1527,1,'2021-10-03 19:23:56.852515','2021-10-03 19:23:56.852517');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (40,400,1242,1,'2021-10-03 19:24:20.76301','2021-10-03 19:24:20.763012');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (41,400,1176,1,'2021-10-03 19:29:35.280057','2021-10-03 19:29:35.280058');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (42,600,721,1,'2021-10-03 19:29:56.829913','2021-10-03 19:29:56.829915');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (43,500,847,1,'2021-10-03 19:30:14.337442','2021-10-03 19:30:14.337444');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (44,200,1399,1,'2021-10-03 19:30:57.21783','2021-10-03 19:30:57.217832');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (45,100,1416,1,'2021-10-03 19:31:10.292414','2021-10-03 19:31:10.292416');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (46,200,827,1,'2021-10-03 19:31:41.560705','2021-10-03 19:31:41.560707');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (47,100,1528,1,'2021-10-03 19:37:45.133791','2021-10-03 19:37:45.133792');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (48,200,1462,1,'2021-10-03 19:42:55.982103','2021-10-03 19:42:55.982105');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (49,600,1529,1,'2021-10-03 19:44:57.181638','2021-10-03 19:44:57.18164');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (50,100,1307,1,'2021-10-03 19:45:16.09898','2021-10-03 19:45:16.098982');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (51,200,1241,1,'2021-10-03 19:46:09.195615','2021-10-03 19:46:09.195617');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (52,100,1531,1,'2021-10-03 19:48:29.822776','2021-10-03 19:48:29.822777');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (53,100,1530,1,'2021-10-03 19:48:50.121963','2021-10-03 19:48:50.121965');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (54,1800,830,1,'2021-10-03 19:49:31.297478','2021-10-03 19:49:31.29748');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (55,200,1532,1,'2021-10-03 19:51:36.970757','2021-10-03 19:51:36.970759');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (56,500,1533,1,'2021-10-03 20:28:36.21422','2021-10-03 20:28:36.214221');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (57,300,1534,1,'2021-10-03 20:29:34.702636','2021-10-03 20:29:34.702638');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (58,100,1535,1,'2021-10-03 20:53:51.479825','2021-10-03 20:53:51.479827');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (61,3000,1536,1,'2021-10-04 02:45:05.236115','2021-10-04 02:45:05.236117');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (62,300,1537,1,'2021-10-04 02:50:43.291283','2021-10-04 02:50:43.291285');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (63,20000,1538,1,'2021-10-04 18:52:04.445081','2021-10-04 18:52:04.445085');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (64,7000,1539,1,'2021-10-04 18:56:45.668015','2021-10-04 18:56:45.66802');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (65,300,1347,1,'2021-10-04 20:04:03.965392','2021-10-04 20:04:03.965393');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (66,7000,1540,1,'2021-10-04 20:16:36.469259','2021-10-04 20:16:36.469264');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (67,10000,1541,1,'2021-10-04 20:21:16.409346','2021-10-04 20:21:16.409351');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (68,600,1544,1,'2021-10-04 20:40:00.210038','2021-10-04 20:40:00.21004');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (69,2000,1545,1,'2021-10-04 20:50:22.272943','2021-10-04 20:50:22.272945');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (70,800,1546,1,'2021-10-05 16:33:53.390542','2021-10-05 16:33:53.390543');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (71,4000,1547,1,'2021-10-05 18:34:45.227259','2021-10-05 18:34:45.227261');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (72,5000,1548,1,'2021-10-05 18:39:11.596574','2021-10-05 18:39:11.596579');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (75,200,1552,1,'2021-10-05 19:01:10.966687','2021-10-05 19:01:10.966689');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (73,5000,1549,1,'2021-10-05 18:41:07.321424','2021-10-05 18:41:07.321425');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (74,1200,1550,1,'2021-10-05 18:45:59.181937','2021-10-05 18:45:59.181939');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (76,500,1551,1,'2021-10-05 19:01:30.985845','2021-10-05 19:01:30.985847');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (77,1200,1554,1,'2021-10-05 19:01:56.382335','2021-10-05 19:01:56.382336');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (78,100,1553,1,'2021-10-05 19:02:12.525301','2021-10-05 19:02:12.525303');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (79,900,1104,1,'2021-10-05 19:05:28.836547','2021-10-05 19:05:28.836549');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (80,500,1555,1,'2021-10-05 19:14:37.895687','2021-10-05 19:14:37.895689');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (81,2000,1556,1,'2021-10-05 19:22:22.063208','2021-10-05 19:22:22.06321');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (82,500,1557,1,'2021-10-05 20:44:24.783159','2021-10-05 20:44:24.78316');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (83,1800,1558,1,'2021-10-05 21:02:34.758421','2021-10-05 21:02:34.758423');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (84,10000,1559,1,'2021-10-05 21:08:37.007053','2021-10-05 21:08:37.007055');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (85,100,1561,1,'2021-10-05 21:12:50.973659','2021-10-05 21:12:50.97366');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (86,100,1560,1,'2021-10-05 21:13:06.020064','2021-10-05 21:13:06.020066');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (87,500,1564,1,'2021-10-05 21:22:44.098102','2021-10-05 21:22:44.098103');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (88,100,1565,1,'2021-10-05 21:38:52.711724','2021-10-05 21:38:52.711725');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (89,200,1569,1,'2021-10-05 21:51:37.504227','2021-10-05 21:51:37.504228');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (90,400,1567,1,'2021-10-05 21:53:33.274781','2021-10-05 21:53:33.274783');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (91,400,1566,1,'2021-10-05 21:54:01.945575','2021-10-05 21:54:01.945577');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (92,400,1568,1,'2021-10-05 21:54:30.0494','2021-10-05 21:54:30.049402');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (93,200,1572,1,'2021-10-05 21:55:01.74457','2021-10-05 21:55:01.744572');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (94,300,1571,1,'2021-10-05 21:55:31.622864','2021-10-05 21:55:31.622866');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (95,400,1570,1,'2021-10-05 21:56:34.685398','2021-10-05 21:56:34.685399');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (96,200,1573,1,'2021-10-05 22:03:05.86526','2021-10-05 22:03:05.865262');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (97,300,1574,1,'2021-10-05 22:12:22.664501','2021-10-05 22:12:22.664502');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (98,14400,1576,1,'2021-10-05 22:20:08.983651','2021-10-05 22:20:08.983652');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (99,900,1577,1,'2021-10-05 22:37:11.070509','2021-10-05 22:37:11.070511');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (100,400,1578,1,'2021-10-05 22:37:28.590135','2021-10-05 22:37:28.590137');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (101,300,1575,1,'2021-10-05 22:41:09.371355','2021-10-05 22:41:09.371356');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (102,300,1579,1,'2021-10-05 22:46:06.028089','2021-10-05 22:46:06.028091');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (103,300,1580,1,'2021-10-05 22:48:48.959399','2021-10-05 22:48:48.959401');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (104,300,1581,1,'2021-10-05 23:01:11.34243','2021-10-05 23:01:11.342431');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (105,400,1582,1,'2021-10-05 23:08:14.74758','2021-10-05 23:08:14.747582');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (106,300,1583,1,'2021-10-05 23:11:13.635508','2021-10-05 23:11:13.63551');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (107,300,1584,1,'2021-10-05 23:15:45.246211','2021-10-05 23:15:45.246212');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (108,300,1585,1,'2021-10-05 23:19:44.484707','2021-10-05 23:19:44.484708');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (109,300,1586,1,'2021-10-05 23:26:49.720476','2021-10-05 23:26:49.720478');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (110,100,1587,1,'2021-10-05 23:31:12.559341','2021-10-05 23:31:12.559343');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (111,900,1588,1,'2021-10-05 23:35:43.664215','2021-10-05 23:35:43.664217');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (112,2400,1453,1,'2021-10-05 23:39:23.555525','2021-10-05 23:39:23.555527');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (113,1100,1589,1,'2021-10-05 23:47:09.352564','2021-10-05 23:47:09.352566');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (114,1200,1590,1,'2021-10-05 23:51:25.236312','2021-10-05 23:51:25.236314');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (115,300,1591,1,'2021-10-06 00:02:54.94618','2021-10-06 00:02:54.946182');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (116,200,1592,1,'2021-10-06 00:04:44.54559','2021-10-06 00:04:44.545592');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (117,600,1593,1,'2021-10-06 00:15:43.415282','2021-10-06 00:15:43.415283');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (118,600,1594,1,'2021-10-06 00:16:00.71254','2021-10-06 00:16:00.712541');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (119,800,1595,1,'2021-10-06 00:16:29.083222','2021-10-06 00:16:29.083224');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (120,7400,921,1,'2021-10-06 22:27:58.084468','2021-10-06 22:27:58.08447');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (121,3700,920,1,'2021-10-06 22:28:50.763032','2021-10-06 22:28:50.763034');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (122,3600,922,1,'2021-10-06 22:29:33.496147','2021-10-06 22:29:33.496149');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (123,500,1118,1,'2021-10-06 22:37:08.555054','2021-10-06 22:37:08.555059');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (124,1500,1119,1,'2021-10-06 22:37:37.54873','2021-10-06 22:37:37.548732');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (125,1000,1120,1,'2021-10-06 22:38:02.862455','2021-10-06 22:38:02.862456');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (126,1700,1286,1,'2021-10-06 22:50:16.513259','2021-10-06 22:50:16.51326');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (127,600,1596,1,'2021-10-06 22:59:56.069375','2021-10-06 22:59:56.069378');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (128,900,1597,1,'2021-10-06 23:00:14.228551','2021-10-06 23:00:14.228552');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (129,500,1598,1,'2021-10-06 23:07:56.538254','2021-10-06 23:07:56.538261');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (130,900,1599,1,'2021-10-06 23:08:18.544075','2021-10-06 23:08:18.544078');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (131,1200,1606,1,'2021-10-06 23:37:45.153473','2021-10-06 23:37:45.153476');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (132,1200,1608,1,'2021-10-06 23:37:58.30468','2021-10-06 23:37:58.304686');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (133,1200,1607,1,'2021-10-06 23:38:10.765898','2021-10-06 23:38:10.7659');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (134,1200,1609,1,'2021-10-07 02:30:10.119509','2021-10-07 02:30:10.11951');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (6,500,1513,1,'2021-10-01 23:03:17.879886','2021-10-01 23:03:17.879888');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (1,2000,905,1,'2021-05-04 14:05:49.389078','2021-05-04 14:05:49.389078');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (135,1200,1609,1,'2021-10-08 23:08:26.637536','2021-10-08 23:08:26.637538');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (136,1200,1610,1,'2021-10-08 23:11:39.622392','2021-10-08 23:11:39.622395');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (137,600,1122,1,'2021-10-08 23:12:43.343203','2021-10-08 23:12:43.343204');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (138,300,1122,1,'2021-10-08 23:23:08.012906','2021-10-08 23:23:08.012908');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (139,1100,1123,1,'2021-10-08 23:24:18.452243','2021-10-08 23:24:18.452245');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (140,1700,1611,1,'2021-10-08 23:46:39.810729','2021-10-08 23:46:39.810731');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (141,2400,1612,1,'2021-10-08 23:49:16.122874','2021-10-08 23:49:16.122876');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (142,100,1620,1,'2021-10-09 00:08:53.235221','2021-10-09 00:08:53.235222');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (143,100,1621,1,'2021-10-09 00:09:13.491477','2021-10-09 00:09:13.491479');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (144,500,1619,1,'2021-10-09 00:09:39.519531','2021-10-09 00:09:39.519532');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (145,100,1614,1,'2021-10-09 00:10:42.251548','2021-10-09 00:10:42.25155');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (146,100,1616,1,'2021-10-09 00:11:08.843328','2021-10-09 00:11:08.843329');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (147,100,1615,1,'2021-10-09 00:11:29.265786','2021-10-09 00:11:29.265788');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (148,100,1615,1,'2021-10-09 00:11:45.517437','2021-10-09 00:11:45.517442');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (149,200,1616,1,'2021-10-09 00:12:05.3654','2021-10-09 00:12:05.365405');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (150,100,1617,1,'2021-10-09 00:12:27.934104','2021-10-09 00:12:27.934106');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (151,100,1618,1,'2021-10-09 00:12:45.157536','2021-10-09 00:12:45.157538');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (152,100,1623,1,'2021-10-09 23:58:04.75721','2021-10-09 23:58:04.757212');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (153,6800,1624,1,'2021-10-10 00:07:16.730452','2021-10-10 00:07:16.730457');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (154,2000,1625,1,'2021-10-10 00:12:49.541186','2021-10-10 00:12:49.541188');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (155,100,1626,1,'2021-10-11 20:01:54.831306','2021-10-11 20:01:54.831311');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (156,1000,1627,1,'2021-10-11 20:08:18.11682','2021-10-11 20:08:18.116822');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (157,1000,1628,1,'2021-10-11 20:11:14.243544','2021-10-11 20:11:14.243545');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (158,1200,1629,1,'2021-10-11 21:11:41.665492','2021-10-11 21:11:41.665494');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (159,1500,1631,1,'2021-10-11 21:35:47.819535','2021-10-11 21:35:47.819537');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (160,200,1632,1,'2021-10-11 21:43:12.704751','2021-10-11 21:43:12.704752');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (161,100,1633,1,'2021-10-11 21:55:50.942616','2021-10-11 21:55:50.942618');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (162,0,1634,1,'2021-10-11 21:57:48.107368','2021-10-11 21:57:48.10737');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (163,1200,1635,1,'2021-10-11 22:31:11.921469','2021-10-11 22:31:11.921471');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (164,100,1641,1,'2021-10-11 23:08:35.635063','2021-10-11 23:08:35.635065');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (165,100,1639,1,'2021-10-11 23:08:54.662494','2021-10-11 23:08:54.662495');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (166,100,1640,1,'2021-10-11 23:09:07.987589','2021-10-11 23:09:07.987591');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (167,100,1643,1,'2021-10-11 23:09:24.283462','2021-10-11 23:09:24.283464');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (168,100,1638,1,'2021-10-11 23:09:41.941174','2021-10-11 23:09:41.941176');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (169,100,1636,1,'2021-10-11 23:09:58.031568','2021-10-11 23:09:58.03157');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (170,100,1637,1,'2021-10-11 23:10:11.619915','2021-10-11 23:10:11.619916');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (171,100,1642,1,'2021-10-11 23:10:34.605211','2021-10-11 23:10:34.605213');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (172,100,1646,1,'2021-10-11 23:58:22.181098','2021-10-11 23:58:22.181099');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (173,100,1647,1,'2021-10-11 23:58:41.450039','2021-10-11 23:58:41.450041');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (174,100,1648,1,'2021-10-11 23:59:00.831941','2021-10-11 23:59:00.831942');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (175,100,1650,1,'2021-10-12 00:10:45.794808','2021-10-12 00:10:45.794809');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (176,100,1184,1,'2021-10-12 00:14:05.948707','2021-10-12 00:14:05.948708');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (177,300,1651,1,'2021-10-12 00:17:07.110559','2021-10-12 00:17:07.11056');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (178,300,1652,1,'2021-10-12 00:23:51.136567','2021-10-12 00:23:51.136569');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (179,200,1653,1,'2021-10-12 00:25:41.101022','2021-10-12 00:25:41.101023');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (180,100,1654,1,'2021-10-16 21:30:46.458962','2021-10-16 21:30:46.458964');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (181,6000,1655,1,'2021-10-16 21:41:57.131487','2021-10-16 21:41:57.131488');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (182,1200,1656,1,'2021-10-16 23:10:52.907199','2021-10-16 23:10:52.9072');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (183,900,1657,1,'2021-10-16 23:22:43.833472','2021-10-16 23:22:43.833473');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (184,3900,1658,1,'2021-10-16 23:42:07.790514','2021-10-16 23:42:07.790515');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (185,1500,1659,1,'2021-10-16 23:51:03.198633','2021-10-16 23:51:03.198635');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (186,500,1661,1,'2021-10-17 00:03:51.319622','2021-10-17 00:03:51.319623');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (187,900,1660,1,'2021-10-17 00:04:51.280775','2021-10-17 00:04:51.280776');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (188,25000,1662,1,'2021-10-22 21:57:09.399533','2021-10-22 21:57:09.399535');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (189,100,1341,1,'2021-10-22 21:59:31.963537','2021-10-22 21:59:31.963538');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (190,200,1664,1,'2021-10-22 22:04:51.300675','2021-10-22 22:04:51.300677');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (191,400,1665,1,'2021-10-22 22:05:13.880956','2021-10-22 22:05:13.880957');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (192,3100,1346,1,'2021-10-22 22:08:55.664889','2021-10-22 22:08:55.664891');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (193,300,1411,1,'2021-10-22 23:01:59.961272','2021-10-22 23:01:59.961274');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (194,500,1342,1,'2021-10-22 23:03:24.810494','2021-10-22 23:03:24.810496');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (195,200,1631,1,'2021-10-22 23:08:49.751603','2021-10-22 23:08:49.751605');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (196,700,1630,1,'2021-10-22 23:09:05.399227','2021-10-22 23:09:05.399232');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (197,4600,1666,1,'2021-10-22 23:21:17.002264','2021-10-22 23:21:17.002266');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (198,3300,1667,1,'2021-10-22 23:32:46.087738','2021-10-22 23:32:46.087743');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (199,7500,1073,1,'2021-10-23 23:22:22.25685','2021-10-23 23:22:22.256853');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (200,200,1668,1,'2021-10-23 23:31:44.262141','2021-10-23 23:31:44.262143');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (201,300,1669,1,'2021-10-23 23:32:09.359587','2021-10-23 23:32:09.359592');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (202,200,1670,1,'2021-10-24 15:06:57.70249','2021-10-24 15:06:57.702492');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (203,2000,1559,1,'2021-10-24 15:07:26.274788','2021-10-24 15:07:26.27479');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (204,400,1671,1,'2021-10-24 16:12:06.689874','2021-10-24 16:12:06.689875');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (205,1300,1672,1,'2021-10-25 13:47:19.98383','2021-10-25 13:47:19.983832');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (206,1000,1673,1,'2021-10-25 13:56:55.715471','2021-10-25 13:56:55.715476');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (207,1000,1674,1,'2021-10-25 13:57:15.135551','2021-10-25 13:57:15.135553');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (208,1000,1675,1,'2021-10-25 14:36:22.08345','2021-10-25 14:36:22.083451');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (209,1000,1676,1,'2021-10-25 14:51:02.541518','2021-10-25 14:51:02.54152');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (210,3400,1677,1,'2021-10-25 15:44:31.535771','2021-10-25 15:44:31.535772');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (211,100,1678,1,'2021-10-25 21:26:12.67001','2021-10-25 21:26:12.670012');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (212,500,1679,1,'2021-10-26 22:36:17.570553','2021-10-26 22:36:17.570555');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (213,8500,1680,1,'2021-10-26 22:46:18.261489','2021-10-26 22:46:18.26149');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (214,4500,1681,1,'2021-10-26 23:07:14.567866','2021-10-26 23:07:14.567868');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (215,3800,1682,1,'2021-10-26 23:20:43.83933','2021-10-26 23:20:43.839332');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (216,700,1630,1,'2021-10-26 23:28:46.398489','2021-10-26 23:28:46.398491');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (217,1200,1683,1,'2021-10-26 23:34:01.074144','2021-10-26 23:34:01.074145');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (218,800,1684,1,'2021-10-26 23:42:14.713463','2021-10-26 23:42:14.713465');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (219,1000,1685,1,'2021-11-01 01:08:11.980192','2021-11-01 01:08:11.980193');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (220,700,1686,1,'2021-11-01 01:13:25.572794','2021-11-01 01:13:25.572795');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (221,3400,1332,1,'2021-11-01 01:19:31.63332','2021-11-01 01:19:31.633322');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (222,6600,1333,1,'2021-11-01 01:34:42.322727','2021-11-01 01:34:42.322728');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (223,2500,1687,1,'2021-11-01 01:52:55.831556','2021-11-01 01:52:55.831558');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (224,1200,1688,1,'2021-11-01 01:57:58.941894','2021-11-01 01:57:58.941896');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (225,300,1105,1,'2021-11-01 02:05:08.003337','2021-11-01 02:05:08.003338');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (226,700,1689,1,'2021-11-01 02:13:09.849865','2021-11-01 02:13:09.849866');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (227,1000,1690,1,'2021-11-01 02:14:57.409418','2021-11-01 02:14:57.40942');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (228,100,1691,1,'2021-11-03 23:30:06.648861','2021-11-03 23:30:06.648862');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (229,200,1692,1,'2021-11-03 23:30:21.982085','2021-11-03 23:30:21.982087');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (230,300,1693,1,'2021-11-03 23:34:38.517055','2021-11-03 23:34:38.517057');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (231,300,1694,1,'2021-11-03 23:34:53.636142','2021-11-03 23:34:53.636144');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (232,900,1104,1,'2021-11-04 02:15:41.121361','2021-11-04 02:15:41.121364');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (233,500,1423,1,'2021-11-04 13:43:24.128202','2021-11-04 13:43:24.128204');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (234,1000,1555,1,'2021-11-04 13:48:37.530916','2021-11-04 13:48:37.530918');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (235,3000,1104,1,'2021-11-04 13:53:42.566755','2021-11-04 13:53:42.566756');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (236,2200,1104,1,'2021-11-04 14:12:29.899403','2021-11-04 14:12:29.899405');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (237,9700,1695,1,'2021-11-04 14:26:56.573181','2021-11-04 14:26:56.573185');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (238,2400,1118,1,'2021-11-04 14:44:01.359421','2021-11-04 14:44:01.359422');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (239,1200,1119,1,'2021-11-04 14:44:32.195423','2021-11-04 14:44:32.195425');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (240,1200,1120,1,'2021-11-04 14:44:43.626951','2021-11-04 14:44:43.626952');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (241,1100,1606,1,'2021-11-04 14:48:35.006785','2021-11-04 14:48:35.006786');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (242,1100,1608,1,'2021-11-04 14:58:18.365852','2021-11-04 14:58:18.365853');
insert into inventariados (id, cantidad, producto_id, inventario_id, created_at, updated_at) values (243,1100,1607,1,'2021-11-04 14:59:03.172944','2021-11-04 14:59:03.172946');

SELECT pg_catalog.setval('public.inventariados_id_seq', 243, true);


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
