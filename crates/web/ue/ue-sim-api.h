
#ifndef UE_SIM_API_H
#define UE_SIM_API_H	1

#include <stdint.h>

#define TSFMT "yyyy-MM-dd HH:mm:ss.SSS"
#define TSLEN sizeof(TSFMT) + 1

#define UNDEF 100

typedef struct {
	char* simulationId;
	char* sceneObjectId;
	long simulationTime;
	long timestamp;
	/** uint8_t timestamp[TSLEN]; */
} sim_msg_header;

//// http消息
// 1.1 model_init

typedef struct {
    uint16_t ueid;
    uint8_t sst;
    uint32_t sd;
    uint8_t cyclicPrefix;
    uint8_t subCarrierSpacing;
    char* routeAddIp;
    uint8_t usimMode;
    uint8_t authAlgo;
    uint8_t opType;
    char opcOrOp[33];
    char k[33];
    char imsi[16];
    char imei[16];
    char msisdn[12];
    char imeisv[17];
    char* dnn;
    int latitude;
    int longitude;
    uint32_t altitude;

} model_init_params;

typedef struct {
	sim_msg_header header;
	model_init_params params;
} model_init_request;

//// kafka 消息

// 1.2 simulation control
typedef enum {
	START = 1,               // 启动
	PAUSE = 2,               // 暂停
	RESUME = 3,              // 恢复
	DOUBLESPEED = 4,         // 倍速
	STOP = 5,                // 停止
} CONTROL_TYPE;

typedef struct {
	// header
	sim_msg_header header;
	
	char* sceneObjectId;
	long simulationTime;
	uint8_t controlType;
	uint8_t speed; // +2: 加速2倍
} sim_control;

typedef struct {
	void (*destroy)(void* p);
	void* data;
} pointer_wrapper;

// http interface
typedef struct {
    uint8_t optType;
    uint32_t capacity;
    char* serviceAddr;
    char* phoneNum;
} model_config_params;

typedef struct {
    uint8_t simFault;
    uint8_t startError;
    uint8_t outOfSync;
} model_simfault_params;

typedef struct {
	sim_msg_header header;
	union {
		model_config_params configParams;
		model_simfault_params simfaultParams;
	};
	int configOrSimfault; // 1: config, 2: simfault
} model_config_request;


// 1.3 environment simulation
typedef struct {
	// header
	sim_msg_header header;
	
	// body
} sim_env;

// 1.4 simlation status output
typedef struct {
	// header
	sim_msg_header header;
	
	// body
	int8_t status;
	char* msg;
	char* containerId;
	char* containerIp;
	int16_t port;
} sim_status_reply;

typedef struct {
	// header
	sim_msg_header header;
	
	// body
	char* sceneObjectId;
	long simulationTime;
	uint8_t rrcLinkStatus;
	uint32_t dlSpeed;
	uint32_t ulSpeed;
	uint32_t timeDelay;
	char* plmn;
	char* ueIp;
	char* ueId;
	int8_t optType;
} sim_status_data;

// 1.5 running logs
typedef enum {
	FATAL = 1,
	ERROR = 2,
	WARN = 3,
	INFO = 4,
} SIM_LOG_LEVEL;

typedef struct {
	// header
	sim_msg_header header;
	
	// body
	char* sceneObjectId;
	long simulationTime;
	SIM_LOG_LEVEL logType;
	char* logDomain;
	char* logData;
} sim_running_log;

// 1.6 model event output
/** typedef enum { */
/**         SUC_CONN = 1, // 接入成功 */
/**         FAI_CONN = 2,     // 接入失败 */
/**         DISCNTD = 3,      // 连接断开 */
/**         STG_SWITCH = 4,   // 开始切换 */
/**         SWTCH_OK = 5,     // 切换成功 */
/**         SWTCH_FAIL = 6,   // 切换失败 */
/**         STG_REBUILD = 7,  // 开始重建 */
/**         REBUIL_OK = 8,    // 重建成功 */
/**         REBUIL_FAIL = 9,  // 重建失败 */
/** } SIM_EVENT_TYPE; */

#define EVENT_TYPE_SUC_CONN "01"      // 接入成功
#define EVENT_TYPE_FAI_CONN "02"     // 接入失败
#define EVENT_TYPE_DISCNTD "03"      // 连接断开
#define EVENT_TYPE_STG_SWITCH "04"   // 开始切换
#define EVENT_TYPE_SWTCH_OK "05"     // 切换成功
#define EVENT_TYPE_SWTCH_FAIL "06"   // 切换失败
#define EVENT_TYPE_STG_REBUILD "07"  // 开始重建
#define EVENT_TYPE_REBUIL_OK "08"    // 重建成功
#define EVENT_TYPE_REBUIL_FAIL "09"  // 重建失败

typedef struct {
	// header
	sim_msg_header header;
	
	// body
	char* sceneObjectId;
	long simulationTime;
	char* eventType;
	char* returnMean;
} sim_model_event;

#ifdef __cplusplus
extern "C" {
#endif

int on_model_init(model_init_request* p_model_init_request);
int on_model_config(model_config_request* p_business_control_request);

int on_sim_control(sim_control* p_sim_control);
int on_sim_env(sim_env* p_sim_control);

int send_sim_status_reply(sim_status_reply* p_sim_status_reply);
int send_sim_status_data(sim_status_data* p_sim_status_data);

int send_running_log(sim_running_log* p_sim_running_log);

int send_model_event(sim_model_event* p_sim_model_event);

#ifdef __cplusplus
}
#endif

#endif
