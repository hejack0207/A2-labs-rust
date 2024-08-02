
#ifndef SIM_API_H
#define SIM_API_H	1

#include <stdint.h>

#define TSFMT "yyyy-MM-dd HH:mm:ss.SSS"
#define TSLEN sizeof(TSFMT) + 1

#define UNDEF 100

#ifndef UE_SIM_API_H
typedef struct {
	char* simulationId;
	char* sceneObjectId;
	long simulationTime;
	long timestamp;
	/** uint8_t timestamp[TSLEN]; */
} sim_msg_header;
#endif

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

int send_sim_status_reply(sim_status_reply* p_sim_status_reply);

int send_sim_status_data(sim_status_data* p_sim_status_data);

int send_running_log(sim_running_log* p_sim_running_log);

int send_model_event(sim_model_event* p_sim_model_event);

#ifdef __cplusplus
}
#endif

#endif
