-- 导入作品数据到数据库
USE graduation_exhibition;

INSERT INTO works (id, title, major_code, major_name, theme, tags, authors, teacher, poster_url, video_url, duration, introduction) VALUES
('w001', '基于LangChain4j的AI网站生成平台设计与实现', 'software', '软件工程', '数智焕新', '["人工智能", "Web应用", "LangChain"]', '["龙文星"]', '王俊杰', '/static/posters/w001.jpg', '/static/videos/w001.mp4', '02:34', '基于LangChain4j框架的AI驱动网站生成平台，通过自然语言描述自动生成网站代码，提升开发效率。'),
('w002', '基于YOLO与大数据技术的交通流量智能分析系统设计与实现', 'software', '软件工程', '数智焕新', '["计算机视觉", "大数据", "YOLO"]', '["陈曾烨"]', '孙佩娟', '/static/posters/w002.jpg', '/static/videos/w002.mp4', '03:12', '结合YOLO目标检测算法与大数据分析技术，实现交通流量的实时监测与智能分析，为城市交通管理提供数据支持。'),
('w003', '基于Vue+Node.js的糖尿病健康咨询系统设计与实现', 'software', '软件工程', '数智焕新', '["Web应用", "医疗健康", "Vue.js"]', '["马小念"]', '罗灿峰', '/static/posters/w003.jpg', '/static/videos/w003.mp4', '02:45', '基于Vue和Node.js技术栈的糖尿病健康咨询系统，为患者提供健康管理、饮食建议和在线咨询服务。'),
('w004', '基于机器学习+Spark的农作物产量分析系统设计与实现', 'software', '软件工程', '数智焕新', '["机器学习", "大数据", "Spark"]', '["李佳欣"]', '张明', '/static/posters/w004.jpg', '/static/videos/w004.mp4', '03:05', '利用机器学习算法和Spark大数据处理框架，对农作物产量进行预测分析，为农业生产提供决策支持。'),
('w005', '基于STM32的智能家居控制系统设计', 'electronic', '电子信息工程', '芯火智造', '["嵌入式", "物联网", "STM32"]', '["王浩然"]', '李强', '/static/posters/w005.jpg', '/static/videos/w005.mp4', '02:50', '基于STM32微控制器的智能家居控制系统，实现家电设备的远程控制和智能联动。'),
('w006', '基于FPGA的图像处理加速器设计', 'electronic', '电子信息工程', '芯火智造', '["FPGA", "图像处理", "硬件加速"]', '["赵雨晴"]', '刘伟', '/static/posters/w006.jpg', '/static/videos/w006.mp4', '03:20', '利用FPGA的并行处理能力，设计高性能图像处理加速器，提升图像处理速度。'),
('w007', '基于LoRa的智能农业监测系统', 'electronic', '电子信息工程', '芯火智造', '["物联网", "LoRa", "传感器"]', '["陈思远"]', '王芳', '/static/posters/w007.jpg', '/static/videos/w007.mp4', '02:55', '采用LoRa低功耗广域网技术，构建智能农业环境监测系统，实现远程数据采集和分析。'),
('w008', '基于深度学习的无人机目标识别系统', 'electronic', '电子信息工程', '芯火智造', '["深度学习", "无人机", "目标识别"]', '["张宇航"]', '赵敏', '/static/posters/w008.jpg', '/static/videos/w008.mp4', '03:15', '结合深度学习算法和无人机平台，实现实时目标识别和跟踪功能。'),
('w009', '基于Unity的虚拟演播室系统设计', 'broadcast', '广播电视工程', '虚实共生', '["虚拟现实", "Unity", "演播室"]', '["刘思琪"]', '陈杰', '/static/posters/w009.jpg', '/static/videos/w009.mp4', '03:30', '利用Unity引擎构建虚拟演播室系统，实现虚拟场景与真实人物的实时合成。'),
('w010', '基于AI的智能视频剪辑系统', 'broadcast', '广播电视工程', '虚实共生', '["人工智能", "视频处理", "自动剪辑"]', '["周雨欣"]', '李娜', '/static/posters/w010.jpg', '/static/videos/w010.mp4', '02:40', '基于人工智能技术的智能视频剪辑系统，自动识别精彩片段并生成短视频。'),
('w011', '基于WebRTC的实时互动直播平台', 'broadcast', '广播电视工程', '虚实共生', '["WebRTC", "直播", "实时通信"]', '["吴俊杰"]', '张华', '/static/posters/w011.jpg', '/static/videos/w011.mp4', '03:10', '基于WebRTC技术的实时互动直播平台，支持低延迟视频传输和多人互动。'),
('w012', '基于AR技术的虚拟导览系统', 'broadcast', '广播电视工程', '虚实共生', '["增强现实", "AR", "导览系统"]', '["林雨涵"]', '王磊', '/static/posters/w012.jpg', '/static/videos/w012.mp4', '02:58', '利用增强现实技术开发虚拟导览系统，为用户提供沉浸式的参观体验。');
