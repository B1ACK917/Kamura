<template>
  <el-container class="kamura-runner" style="height: 800px">
    <el-aside width="400px">
      <el-scrollbar>
        <el-menu>
          <el-menu-item-group>
            <!--            <el-menu-item index="1-1" >Basic Settings</el-menu-item>-->
            <el-menu-item name="1-2">Perseus Settings</el-menu-item>
            <el-menu-item name="1-3">Advanced Settings</el-menu-item>
          </el-menu-item-group>
        </el-menu>
      </el-scrollbar>
    </el-aside>

    <el-main title="Perseus Info">
      <el-descriptions>
        <el-descriptions-item label="Perseus Path">{{ perseusPath }}</el-descriptions-item>
        <el-descriptions-item label="Perseus Version">{{ perseusVersion }}</el-descriptions-item>
      </el-descriptions>
      <el-descriptions>
        <el-descriptions-item label="Perseus Build Date">{{ perseusBuildDate }}</el-descriptions-item>
        <el-descriptions-item label="Status">
          <el-tag :type="perseusValid.type">{{ perseusValid.value }}</el-tag>
        </el-descriptions-item>
      </el-descriptions>
      <el-descriptions>
        <el-descriptions-item label="Update Perseus:">
          <el-button circle color="#d1edc4">
            <el-icon size="small">
              <RefreshLeft/>
            </el-icon>
          </el-button>
        </el-descriptions-item>
      </el-descriptions>
      <el-descriptions>
        <el-descriptions-item label="Rebuild Perseus:">
          <el-button circle color="#d1edc4" @click="submitRebuildPerseus()">
            <el-icon size="small">
              <RefreshLeft/>
            </el-icon>
          </el-button>
          <el-tag :type="perseusRebuildStatus.type" style="margin-left: 16px">
            <el-tooltip
                class="box-item"
                effect="dark"
                :content="perseusRebuildStatus.value"
                placement="top-start"
            >
              {{ perseusRebuildStatus.type }}
            </el-tooltip>
          </el-tag>
        </el-descriptions-item>
      </el-descriptions>
      <el-descriptions>
        <el-descriptions-item label="Rebuild Spike:">
          <el-button circle color="#d1edc4">
            <el-icon size="small">
              <RefreshLeft/>
            </el-icon>
          </el-button>
        </el-descriptions-item>
      </el-descriptions>
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
      perseusBuildDate: null,
      perseusValid: {
        type: 'info',
        value: 'Unknown'
      },
      perseusRebuildStatus: {
        type: 'info',
        value: 'Unknown'
      }
    };
  },
  mounted() {
    this.fetchPerseusInfo();
    this.startRebuildStatusPolling();
  },
  beforeUnmount() {
    clearInterval(this.rebuildStatusInterval);
  },
  methods: {
    async fetchPerseusInfo() {
      try {
        const pathResponse = await axios.get(`${kamura_engine_url}/getPerseus`);
        const versionResponse = await axios.get(`${kamura_engine_url}/getPerseusVersion`);
        const buildDateResponse = await axios.get(`${kamura_engine_url}/getPerseusBuildDate`);
        const validResponse = await axios.get(`${kamura_engine_url}/getPerseusStatus`);

        this.perseusPath = pathResponse.data.message;
        this.perseusVersion = versionResponse.data.message;
        this.perseusValid.value = validResponse.data.message;
        this.perseusValid.type = validResponse.data.success ? 'success' : 'danger';
        this.perseusBuildDate = buildDateResponse.data.message;
      } catch (error) {
        console.error('Failed to fetch Perseus information:', error);
      }
    },
    async fetchRebuildStatus() {
      try {
        const response = await axios.get(`${kamura_engine_url}/getPerseusRebuildStatus`);
        if (response.data.success) {
          this.perseusRebuildStatus.value = response.data.message;
          if (response.data.message === "Running") this.perseusRebuildStatus.type = 'info'
          else this.perseusRebuildStatus.type = 'success'
        } else {
          this.perseusRebuildStatus = {
            type: 'danger',
            value: response.data.message
          };
        }
      } catch (error) {
        console.error('Failed to fetch Perseus rebuild status:', error);
      }
    },
    async submitRebuildPerseus() {
      await axios.get(`${kamura_engine_url}/rebuildPerseus`);
    },
    startRebuildStatusPolling() {
      this.fetchRebuildStatus();
      this.rebuildStatusInterval = setInterval(this.fetchRebuildStatus, 2000);
    }
  }
}
</script>

<style scoped>
</style>
