<template>
  <el-container class="kamura-runner" style="height: 90vh">
    <el-scrollbar>
      <el-radio-group v-model="collapse.left" style="margin-bottom: 20px">
        <el-radio-button :value="true">
          <el-icon>
            <Minus/>
          </el-icon>
        </el-radio-button>
        <el-radio-button :value="false">
          <el-icon>
            <Plus/>
          </el-icon>
        </el-radio-button>
      </el-radio-group>
      <el-menu :collapse="collapse.left">
        <el-sub-menu index="1">
          <template #title>
            <el-icon>
              <DocumentAdd/>
            </el-icon>
            <span>Workloads</span>
          </template>
          <!-- Trace Subgroup -->
          <el-menu-item-group title="Trace">
            <el-menu-item
                v-for="(workload, index) in traceWorkloads"
                :key="'trace-' + index"
                @click="addTaskToRunner(workload[0], workload[1])"
            >
              {{ workload[0] }}
            </el-menu-item>
          </el-menu-item-group>
          <!-- ELF Subgroup -->
          <el-menu-item-group title="Elf">
            <el-menu-item
                v-for="(workload, index) in elfWorkloads"
                :key="'elf-' + index"
                @click="addTaskToRunner(workload[0], workload[1])"
            >
              {{ workload[0] }}
            </el-menu-item>
          </el-menu-item-group>
          <!-- ELSE Subgroup -->
          <el-menu-item-group title="Else">
            <el-menu-item
                v-for="(workload, index) in elseWorkloads"
                :key="'else-' + index"
                @click="addTaskToRunner(workload[0], workload[1])"
            >
              {{ workload[0] }}
            </el-menu-item>
          </el-menu-item-group>
        </el-sub-menu>

      </el-menu>
    </el-scrollbar>

    <el-main>
      <el-space size="large">
        <el-descriptions column="2">
          <el-descriptions-item label="Selected Arch: ">
            <el-tag size="small">{{ this.sharedSelectedArch }}</el-tag>
          </el-descriptions-item>
        </el-descriptions>
      </el-space>
      <div>
        <textarea id="log" readonly></textarea>
      </div>
      <el-space size="large">
        <el-descriptions column="2">
          <el-descriptions-item label="Arch: ">
            <el-tag size="small">{{ logInfo.arch }}</el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="Workload: ">
            <el-tag size="small">{{ logInfo.workload }}</el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="Submit: ">
            <el-tag size="small">{{ logInfo.submitTime }}</el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="Finished: ">
            <el-tag size="small">{{ logInfo.finishedTime }}</el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="Elapsed: ">
            <el-tag size="small">{{ logInfo.elapsed }}</el-tag>
          </el-descriptions-item>
        </el-descriptions>
      </el-space>
    </el-main>

    <el-scrollbar>
      <el-radio-group v-model="collapse.right" style="margin-bottom: 20px">
        <el-radio-button :value="true">
          <el-icon>
            <Minus/>
          </el-icon>
        </el-radio-button>
        <el-radio-button :value="false">
          <el-icon>
            <Plus/>
          </el-icon>
        </el-radio-button>
      </el-radio-group>
      <el-menu :collapse="collapse.right">
        <el-sub-menu index="2">
          <template #title>
            <el-icon>
              <Files/>
            </el-icon>
            <span>Runners</span>
          </template>
          <el-menu-item-group>
            <el-space direction="vertical">
              <el-space v-for="task in tasks" :key="task.uuid" >
                <el-menu-item @click="fetchTaskLog(task.uuid)" style="padding: 0">
                  <el-icon :style="{ color: task.color }">
                    <component :is="task.icon"/>
                  </el-icon>
                  {{ task.uuid }}
                </el-menu-item>
                <el-button type="danger" :icon="Delete" circle @click="removeTask(task.uuid)" size="default"/>
              </el-space>
            </el-space>
          </el-menu-item-group>
        </el-sub-menu>
      </el-menu>
    </el-scrollbar>
  </el-container>
</template>

<script>
import axios from 'axios';
import {kamuraEngineUrl, runnerDefaultTaskInfo} from "@/utils/consts";
import {commonFetchPost} from "@/utils/funcs";
import {Delete} from "@element-plus/icons-vue";

export default {
  name: 'KamuraRunner',
  computed: {
    Delete() {
      return Delete
    }
  },
  props: {
    sharedSelectedArch: {
      type: String, // Adjust the type based on the expected type of `sharedSelectedArch`
      default: null
    }
  },
  data() {
    return {
      collapse: {
        left: false,
        right: false,
      },
      tasks: [],  // Store tasks with uuid, color, icon
      traceWorkloads: [],
      elfWorkloads: [],
      elseWorkloads: [],
      wsList: [],
      selectedArch: null,
      selectedUUID: null,
      logWS: null,
      logInfo: runnerDefaultTaskInfo,
    };
  },
  async created() {
    await this.fetchAllTasksAndPush();
    await this.fetchWorkloads();
    await this.startUpdateTaskStatuses();
  },
  beforeUnmount() {
    for (let ws of this.wsList) ws.close();
    if (this.logWS != null) this.logWS.close();
  },
  methods: {
    async fetchWorkloads() {
      try {
        const response = await axios.get('https://kamura-engine.malloc.fun/getValidWorkloads');
        if (response.data.success) {
          this.traceWorkloads = response.data.workloads.filter(workload => workload[1] === 'trace');
          this.elfWorkloads = response.data.workloads.filter(workload => workload[1] === 'elf');
          this.elseWorkloads = response.data.workloads.filter(workload => workload[1] === 'else');
        } else {
          console.error('Failed to fetch workloads');
        }
      } catch (error) {
        console.error('Error fetching workloads:', error);
      }
    },
    async fetchAllTasksAndPush() {
      try {
        const response = await axios.get(`${kamuraEngineUrl}/getAllTasks`);
        if (response.data.success) {
          const newTasks = response.data.tasks.filter(task => {
            return !this.tasks.some(existingTask => existingTask.uuid === task);
          });

          newTasks.forEach(task => {
            this.tasks.push({
              uuid: task,
              color: 'gray',
              icon: "SuccessFilled"
            });
          });
        }
      } catch (error) {
        console.error('Error fetching tasks:', error);
      }
    },
    async fetchAllTasksAndRefresh() {
      try {
        const response = await axios.get(`${kamuraEngineUrl}/getAllTasks`);
        if (response.data.success) {
          this.tasks = [];
          response.data.tasks.forEach(task => {
            this.tasks.push({
              uuid: task,
              color: 'gray',
              icon: "SuccessFilled"
            });
          });
        }
      } catch (error) {
        console.error('Error fetching tasks:', error);
      }
    },
    async startUpdateTaskStatuses() {
      this.wsList = [];
      for (let task of this.tasks) {
        const ws = new WebSocket(`${kamuraEngineUrl}/ws/getTaskStatus/${task.uuid}`);
        ws.onmessage = (message) => {
          switch (message.data) {
            case "Succeed":
              task.color = "green";
              task.icon = "SuccessFilled";
              break;
            case "Failed":
              task.color = "red";
              task.icon = "CircleCloseFilled";
              break;
            case "Running":
              task.color = "yellow";
              task.icon = "InfoFilled";
              break;
            default:
              task.color = "gray";
              task.icon = "InfoFilled";
              break;
          }
        }
        this.wsList.push(ws);
      }
    },
    async fetchTaskLog(uuid) {
      this.selectedUUID = uuid;
      if (this.logWS != null) this.logWS.close();
      this.fetchTaskInfo(uuid).then();
      this.logWS = new WebSocket(`${kamuraEngineUrl}/ws/getTaskLog/${uuid}`);
      this.logWS.onmessage = (message) => {
        let responseSpan = document.getElementById("log");
        responseSpan.innerHTML = message.data;
      }
    },
    async fetchTaskInfo(uuid) {
      try {
        const data = await commonFetchPost(`${kamuraEngineUrl}/getTaskInfo`, {target: uuid});
        this.logInfo = {
          arch: data.arch,
          workload: data.workload,
          submitTime: data.submit_time,
          finishedTime: data.finished_time,
          elapsed: data.elapsed
        }
      } catch (error) {
        console.error("Error fetching log info:", error);
      }
    },
    async addTaskToRunner(workload, workloadType) {
      try {
        const response = await axios.post(`${kamuraEngineUrl}/addTask`, {
          arch: this.sharedSelectedArch,
          workload: workload,
          workload_type: workloadType
        });
        console.log('Response:', response.data);
      } catch (error) {
        console.error('Error:', error);
      }
      await this.fetchAllTasksAndPush();
      await this.startUpdateTaskStatuses();
    },
    async removeTask(uuid) {
      await commonFetchPost(`${kamuraEngineUrl}/removeTask`, {target: uuid});
      await this.fetchAllTasksAndRefresh();
      for (let ws of this.wsList) ws.close();
      await this.startUpdateTaskStatuses();
      this.logInfo = runnerDefaultTaskInfo;
      if (this.logWS != null) this.logWS.close();
      let responseSpan = document.getElementById("log");
      responseSpan.innerHTML = "";
    }
  },
}
</script>

<style scoped>
#log {
  width: 98%;
  height: 700px;
  margin: 0 auto;
  padding: 10px;
  text-align: left;
  border-radius: 4px;
  overflow-x: hidden;
  overflow-y: auto;
}
</style>
