
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

#ifdef __cplusplus
extern "C" {
#endif

int on_model_init(model_init_request* p_model_init_request);
int on_model_config(model_config_request* p_business_control_request);

int on_sim_control(sim_control* p_sim_control);
int on_sim_env(sim_env* p_sim_control);

#ifdef __cplusplus
}
#endif

#endif
