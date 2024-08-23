#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

#include "ue-sim-api.h"
#include "sim-api.h"

int on_model_init(model_init_request* p_model_init_request){
        printf("\non model init\n");
        printf("simulationId: %s\n", p_model_init_request->header.simulationId);
        printf("sceneObjectId: %s\n", p_model_init_request->header.sceneObjectId);
	return 0;
}

int on_model_config(model_config_request* p_business_control_request){
        printf("\non model config\n");
	printf("simulationId=%s\n",p_business_control_request->header.simulationId);

        sim_status_reply *reply = calloc(1, sizeof(sim_status_reply));
        if (reply != NULL) {
            reply->status = 0;
            reply->containerId = "uesim";
            reply->containerIp = "192.168.0.1";
            reply->msg = "msg from C";
            reply->port = 3333;
            reply->header.simulationId = p_business_control_request->header.simulationId;
            reply->header.sceneObjectId = p_business_control_request->header.sceneObjectId;
            reply->header.simulationTime = p_business_control_request->header.simulationTime;
            reply->header.timestamp = p_business_control_request->header.timestamp;
            send_sim_status_reply(reply);

            free(reply);
        }else{
            printf("calloc failed\n");
        }

	return 0;
}

int on_sim_control(sim_control* p_sim_control){
        return 0;
}

int on_sim_env(sim_env* p_sim_control){
	return 0;
}

