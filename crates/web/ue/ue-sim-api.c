#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "ue-sim-api.h"
#include "sim-api.h"

int on_model_init(model_init_request* p_model_init_request){
        printf("simulationId: %s\n", p_model_init_request->header.simulationId);
        printf("sceneObjectId: %s\n", p_model_init_request->header.sceneObjectId);
	return 0;
}

int on_model_config(model_config_request* p_business_control_request){
	printf("model config got: simulationId=%s\n",p_business_control_request->header.simulationId);

	sim_status_reply *reply = calloc(1, sizeof(sim_status_reply));
	reply->status = 0;
	reply->containerId = "uesim";
	send_sim_status_reply(reply);

	return 0;
}

int on_sim_control(sim_control* p_sim_control){
        return 0;
}

int on_sim_env(sim_env* p_sim_control){
	return 0;
}

