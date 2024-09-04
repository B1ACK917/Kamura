<template>
  <el-container class="kamura-runner" style="height: 80vh">
    <el-aside width="400px">
      <el-scrollbar>
        <el-menu>
          <el-menu-item-group>
            <el-menu-item name="1-2">Perseus Settings</el-menu-item>
            <el-menu-item name="1-3">Advanced Settings</el-menu-item>
          </el-menu-item-group>
        </el-menu>
      </el-scrollbar>
    </el-aside>

    <el-main title="Perseus Info" style="border-collapse: separate" align="left">
      <el-space direction="vertical" alignment="normal" size="large">
        <el-descriptions>
          <el-descriptions-item label="Status">
            <el-tag :type="perseusValid.type">{{ perseusValid.value }}</el-tag>
          </el-descriptions-item>
        </el-descriptions>
        <el-descriptions>
          <el-descriptions-item label="Perseus Path">{{ perseusPath }}</el-descriptions-item>
        </el-descriptions>
        <el-descriptions>
          <el-descriptions-item label="Perseus Version">{{ perseusVersion }}</el-descriptions-item>
        </el-descriptions>
        <el-descriptions>
          <el-descriptions-item label="Perseus Version Date">{{ perseusVersionDate }}</el-descriptions-item>
        </el-descriptions>
        <el-descriptions>
          <el-descriptions-item label="Perseus Build Date">{{ perseusBuildDate }}</el-descriptions-item>
        </el-descriptions>
        <el-descriptions>
          <el-descriptions-item label="Spike Build Date">{{ spikeBuildDate }}</el-descriptions-item>
        </el-descriptions>
        <el-descriptions>
          <el-descriptions-item label="Update Perseus:">
            <el-button circle color="#d1edc4" @click="submitUpdatePerseus()">
              <el-icon size="large">
                <Refresh/>
              </el-icon>
            </el-button>
            <el-tag :type="perseusUpdateStatus.type" style="margin-left: 16px">
              <el-tooltip
                  class="box-item"
                  effect="dark"
                  :content="perseusUpdateStatus.message"
                  placement="top-start"
              >
                {{ perseusUpdateStatus.value }}
              </el-tooltip>
            </el-tag>
          </el-descriptions-item>
        </el-descriptions>
        <el-descriptions>
          <el-descriptions-item label="Rebuild Perseus:">
            <el-button circle color="#d1edc4" @click="submitRebuildPerseus()">
              <el-icon size="large">
                <Refresh/>
              </el-icon>
            </el-button>
            <el-tag :type="perseusRebuildStatus.type" style="margin-left: 16px">
              <el-tooltip
                  class="box-item"
                  effect="dark"
                  :content="perseusRebuildStatus.message"
                  placement="top-start"
              >
                {{ perseusRebuildStatus.value }}
              </el-tooltip>
            </el-tag>
          </el-descriptions-item>
        </el-descriptions>
        <el-descriptions>
          <el-descriptions-item label="Rebuild Spike:">
            <el-button circle color="#d1edc4" @click="submitRebuildSpike()">
              <el-icon size="large">
                <Refresh/>
              </el-icon>
            </el-button>
            <el-tag :type="spikeRebuildStatus.type" style="margin-left: 16px">
              <el-tooltip
                  class="box-item"
                  effect="dark"
                  :content="spikeRebuildStatus.message"
                  placement="top-start"
              >
                {{ spikeRebuildStatus.value }}
              </el-tooltip>
            </el-tag>
          </el-descriptions-item>
        </el-descriptions>
      </el-space>
    </el-main>
  </el-container>
</template>

<script>
import {kamura_engine_url} from "@/utils/consts";
import axios from "axios";

export default {
  name: 'KamuraSettings',
  data() {
    return {
      activeName: '1-2',
      perseusPath: null,
      perseusVersion: null,
      perseusVersionDate: null,
      perseusBuildDate: null,
      spikeBuildDate: null,
      perseusValid: {
        type: 'info',
        value: 'Unknown'
      },
      perseusRebuildStatus: {
        type: 'info',
        value: 'Unknown',
        message: ''
      },
      perseusUpdateStatus: {
        type: 'info',
        value: 'Unknown',
        message: ''
      },
      spikeRebuildStatus: {
        type: 'info',
        value: 'Unknown',
        message: ''
      },
      wsList: []
    };
  },
  mounted() {
    this.fetchPerseusInfo();
    this.startUpdateAllStatus();
  },
  beforeUnmount() {
    for (let ws of this.wsList) {
      ws.close();
    }
  },
  methods: {
    async fetchPerseusInfo() {
      try {
        const validResponse = await axios.get(`${kamura_engine_url}/getPerseusStatus`);
        this.perseusValid.value = validResponse.data.message;
        this.perseusValid.type = validResponse.data.success ? 'success' : 'danger';

        const pathResponse = await axios.get(`${kamura_engine_url}/getPerseus`);
        this.perseusPath = pathResponse.data.message;

        const versionResponse = await axios.get(`${kamura_engine_url}/getPerseusVersion`);
        this.perseusVersion = versionResponse.data.message;

        const versionDateResponse = await axios.get(`${kamura_engine_url}/getPerseusDate`);
        this.perseusVersionDate = versionDateResponse.data.message;

        const perseusBuildDateResponse = await axios.post(`${kamura_engine_url}/getBuildDate`, {target: "Perseus"});
        this.perseusBuildDate = perseusBuildDateResponse.data.message;

        const spikeBuildDateResponse = await axios.post(`${kamura_engine_url}/getBuildDate`, {target: "Spike"});
        this.spikeBuildDate = spikeBuildDateResponse.data.message;
      } catch (error) {
        console.error('Failed to fetch Perseus information:', error);
      }
    },
    async submitRebuildPerseus() {
      await axios.get(`${kamura_engine_url}/rebuildPerseus`);
    },
    async submitUpdatePerseus() {
      await axios.get(`${kamura_engine_url}/updatePerseus`);
    },
    async submitRebuildSpike() {
      await axios.get(`${kamura_engine_url}/rebuildSpike`);
    },
    async startUpdateStatus(updateObj, api) {
      const ws = new WebSocket(`${kamura_engine_url}/ws/${api}`);
      ws.onmessage = (message) => {
        const data = JSON.parse(message.data);
        switch (data.success) {
          case true:
            switch (data.message) {
              case 'Succeed':
                updateObj.type = "success";
                updateObj.value = "Success";
                updateObj.message = data.message;
                break;
              case 'Running':
                updateObj.type = "info";
                updateObj.value = "Progress";
                updateObj.message = data.message;
                break;
            }
            break;
          default:
            updateObj.type = "danger";
            updateObj.value = "Failed";
            updateObj.message = data.message;
            break;
        }
      }
      this.wsList.push(ws);
    },
    startUpdateAllStatus() {
      this.startUpdateStatus(this.perseusRebuildStatus, "getPerseusRebuildStatus");
      this.startUpdateStatus(this.perseusUpdateStatus, "getPerseusUpdateStatus");
      this.startUpdateStatus(this.spikeRebuildStatus, "getSpikeRebuildStatus");
    }
  }
}
</script>

<style scoped>
</style>
