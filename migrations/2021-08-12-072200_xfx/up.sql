-- xfx监控数据
DROP TABLE IF EXISTS `xfx_server_system_new`;

CREATE TABLE `xfx_server_system_new` (
    `tid` bigint NOT NULL AUTO_INCREMENT COMMENT 'tempid',
    `id` int(10) unsigned NOT NULL DEFAULT '0' COMMENT 'xfx_server_system表ID',
    `server_id` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '服务器ID',
    `customer_id` bigint NOT NULL DEFAULT '0' COMMENT '矿机所属客户ID',
    `ip` varchar(128) DEFAULT NULL COMMENT 'ip',
    `plots` bigint NOT NULL COMMENT '文件数',
    `spaces` bigint NOT NULL COMMENT '算力默认单位GB。  1024  TB  1024  PB',
    `local_best_height` bigint NOT NULL COMMENT '本地区块高度',
    `peer_count` bigint NOT NULL COMMENT '只记录',
    `state` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态：1：Farming，正在收割 2: Syncing,正在同步  3:其它状态',
    `state_str` varchar(150) DEFAULT NULL COMMENT '状态字符',
    `mining` tinyint(1) NOT NULL COMMENT '是否在探矿true，客户端状态，先不输出',
    `syncing` tinyint(1) NOT NULL COMMENT '是否在同步false，先不输出',
    `version` varchar(150) DEFAULT NULL COMMENT '版本号',
    `update_timestamp` BIGINT DEFAULT NULL COMMENT 'json脚本上的时间',
    `update_time` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT 'json插到mysql时间',
    PRIMARY KEY (`tid`),
    KEY `id` (`id`),
    KEY `ip` (`ip`),
    KEY `server_id` (`server_id`),
    KEY `customer_id` (`customer_id`),
    KEY `update_timestamp` (`update_timestamp`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8 COMMENT = 'xfx主机最新信息表';

DROP TABLE IF EXISTS `xfx_server_system`;

CREATE TABLE `xfx_server_system` (
    `id` bigint NOT NULL AUTO_INCREMENT,
    `server_id` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '服务器ID',
    `customer_id` bigint NOT NULL DEFAULT '0' COMMENT '矿机所属客户ID',
    `ip` varchar(128) DEFAULT NULL COMMENT 'ip',
    `plots` bigint NOT NULL COMMENT '文件数',
    `spaces` bigint NOT NULL COMMENT '算力默认单位GB。  1024  TB  1024  PB',
    `local_best_height` bigint NOT NULL COMMENT '本地区块高度',
    `peer_count` bigint NOT NULL COMMENT '只记录',
    `state` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态：1：Farming，正在收割 2: Syncing,正在同步  3:其它状态',
    `state_str` varchar(150) DEFAULT NULL COMMENT '状态字符',
    `mining` tinyint(1) NOT NULL COMMENT '是否在探矿true，客户端状态，先不输出',
    `syncing` tinyint(1) NOT NULL COMMENT '是否在同步false，先不输出',
    `version` varchar(150) DEFAULT NULL COMMENT '版本号',
    `update_timestamp` BIGINT DEFAULT NULL COMMENT 'json脚本上的时间',
    `update_time` datetime NOT NULL DEFAULT current_timestamp() COMMENT 'json插到mysql时间',
    PRIMARY KEY (`id`),
    KEY `ip` (`ip`),
    KEY `server_id` (`server_id`),
    KEY `customer_id` (`customer_id`),
    KEY `update_timestamp` (`update_timestamp`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8 COMMENT = 'xfx主机无脑直插表';