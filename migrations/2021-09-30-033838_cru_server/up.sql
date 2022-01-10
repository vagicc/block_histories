-- 这里要增加记录矿机地址，   矿机表也要增加一个矿机地址
-- DROP TABLE IF EXISTS `cru_server_system`;
CREATE TABLE `cru_server_system` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `server_id` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '服务器ID',
  `customer_id` bigint NOT NULL DEFAULT '0' COMMENT '矿机所属客户ID',
  `ip1` varchar(128) DEFAULT NULL COMMENT 'ip',
  `ip2` varchar(128) DEFAULT NULL COMMENT 'ip',

  `group_owner` varchar(150) DEFAULT NULL COMMENT 'CRU账户Stash',
  `account_id` varchar(150) DEFAULT NULL COMMENT '矿机地址',
  `cap`varchar(150) DEFAULT NULL COMMENT '存储容量:335176150962775 =>304.84TB',
  `used`varchar(150) DEFAULT NULL COMMENT '已用存储容量:4419375703=>4.115GB',
  `spare`varchar(150) DEFAULT NULL COMMENT '可用存储容量 :335171731587072=>304.836TB',
  `report_slot` bigint NOT NULL DEFAULT 0  COMMENT '',
  `reported_files_size`varchar(150) DEFAULT NULL COMMENT '',

  -- `stash` varchar(150) DEFAULT NULL COMMENT 'CRU账户Stash',
  -- `controller` varchar(150) DEFAULT NULL COMMENT '控制账号',
  -- `generated_at` bigint NOT NULL DEFAULT 0 COMMENT '时间戳',
  `generated_at` BIGINT DEFAULT NULL COMMENT '生成时间戳',

  -- `srd_complete` bigint NOT NULL DEFAULT 0 COMMENT '已封装',
  -- `srd_remaining_task` bigint NOT NULL DEFAULT 0 COMMENT '剩余工作量',
  -- `disk_available_for_srd` bigint NOT NULL DEFAULT 0 COMMENT '总工作量',
  -- `disk_available` bigint NOT NULL DEFAULT 0 COMMENT '硬盘可用',
  -- `disk_volume` bigint NOT NULL DEFAULT 0 COMMENT '硬盘剩余量',
  -- `sys_disk_available` bigint NOT NULL DEFAULT 0 COMMENT '硬盘已使用',
  -- `disk_count` int NOT NULL DEFAULT 0 COMMENT '硬盘数量',

  `chain_status` varchar(10) DEFAULT 'running' COMMENT '链状态：为running时正常',
  `api_status` varchar(10) DEFAULT 'running' COMMENT 'api状态：为running时正常',
  `sworker_status` varchar(10) DEFAULT 'running' COMMENT '状态：为running时正常',
  `smanager_status` varchar(10) DEFAULT 'running' COMMENT '状态：为running时正常',
  `ipfs_status` varchar(10) DEFAULT 'running' COMMENT '状态：为running时正常',

  `create_time` datetime NOT NULL DEFAULT current_timestamp() COMMENT 'json插到mysql时间',
  PRIMARY KEY (`id`),
  KEY `server_id` (`server_id`),
  KEY `group_owner` (`group_owner`),
  KEY `account_id` (`account_id`),
  KEY `generated_at` (`generated_at`),
  KEY `customer_id` (`customer_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8 COMMENT = 'CRU矿机无脑直插表';

-- DROP TABLE IF EXISTS `cru_server_system_new`;
CREATE TABLE `cru_server_system_new` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'tempid',
  `server_id` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '服务器ID',
  `customer_id` bigint NOT NULL DEFAULT '0' COMMENT '矿机所属客户ID',
  `ip1` varchar(128) DEFAULT NULL COMMENT 'ip',
  `ip2` varchar(128) DEFAULT NULL COMMENT 'ip',

  `group_owner` varchar(150) DEFAULT NULL COMMENT 'CRU账户Stash',
  `account_id` varchar(150) DEFAULT NULL COMMENT '矿机地址',
  `cap`varchar(150) DEFAULT NULL COMMENT '存储容量:335176150962775 =>304.84TB',
  `used`varchar(150) DEFAULT NULL COMMENT '已用存储容量:4419375703=>4.115GB',
  `spare`varchar(150) DEFAULT NULL COMMENT '可用存储容量 :335171731587072=>304.836TB',
  `report_slot` bigint NOT NULL DEFAULT 0  COMMENT '',
  `reported_files_size`varchar(150) DEFAULT NULL COMMENT '',

  `generated_at` BIGINT DEFAULT NULL COMMENT '生成时间戳',

  -- `srd_complete` bigint NOT NULL DEFAULT 0 COMMENT '已封装',
  -- `srd_remaining_task` bigint NOT NULL DEFAULT 0 COMMENT '剩余工作量',
  -- `disk_available_for_srd` bigint NOT NULL DEFAULT 0 COMMENT '总工作量',
  -- `disk_available` bigint NOT NULL DEFAULT 0 COMMENT '硬盘可用',
  -- `disk_volume` bigint NOT NULL DEFAULT 0 COMMENT '硬盘剩余量',
  -- `sys_disk_available` bigint NOT NULL DEFAULT 0 COMMENT '硬盘已使用',
  -- `disk_count` int NOT NULL DEFAULT 0 COMMENT '硬盘数量',

  `chain_status` varchar(10) DEFAULT 'running' COMMENT '链状态：为running时正常',
  `api_status` varchar(10) DEFAULT 'running' COMMENT 'api状态：为running时正常',
  `sworker_status` varchar(10) DEFAULT 'running' COMMENT '状态：为running时正常',
  `smanager_status` varchar(10) DEFAULT 'running' COMMENT '状态：为running时正常',
  `ipfs_status` varchar(10) DEFAULT 'running' COMMENT '状态：为running时正常',
  `update_time` datetime NOT NULL DEFAULT current_timestamp() COMMENT 'json插到mysql时间',
  PRIMARY KEY (`id`),
  KEY `server_id` (`server_id`),
  KEY `customer_id` (`customer_id`),
  KEY `group_owner` (`group_owner`),
  KEY `account_id` (`account_id`),

  KEY `chain_status` (`chain_status`),
  KEY `api_status` (`api_status`),
  KEY `sworker_status` (`sworker_status`),
  KEY `smanager_status` (`smanager_status`),
  KEY `ipfs_status` (`ipfs_status`),
  KEY `generated_at` (`generated_at`),
  KEY `update_time` (`update_time`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8 COMMENT = 'CRU矿机最新信息表';