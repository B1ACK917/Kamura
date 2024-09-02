<template>
  <el-container class="kamura-runner" style="height: 800px">
    <el-aside width="400px">
      <el-scrollbar>
        <el-menu>
          <el-sub-menu index="1">
            <template #title>
              Workloads
            </template>
            <el-menu-item-group>
              <el-menu-item index="1-1" @click="addTaskToRunner('hello_trace','trace')">hello_trace</el-menu-item>
              <el-menu-item index="1-2" @click="addTaskToRunner('dhry_riscv','trace')">dhry_riscv</el-menu-item>
              <el-menu-item index="1-3" @click="addTaskToRunner('core_riscv','trace')">core_riscv</el-menu-item>
              <el-menu-item index="1-4" @click="addTaskToRunner('dependency-p','elf')">dependency-p</el-menu-item>
              <el-menu-item index="1-5" @click="addTaskToRunner('store-p','elf')">store-p</el-menu-item>
              <el-menu-item index="1-6" @click="addTaskToRunner('store_simple-p','elf')">store_simple-p</el-menu-item>
              <el-menu-item index="1-7" @click="addTaskToRunner('test-p','elf')">test-p</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>
          <el-sub-menu index="2">
            <template #title>
              Runners
            </template>
            <el-menu-item-group>
              <el-menu-item v-for="task in tasks" :key="task.uuid" @click="fetchTaskLog(task.uuid)">
                <el-icon :style="{ color: getStatusColor(task.status) }">
                  <component :is="getIconComponent(task.status)"/>
                </el-icon>
                {{ task.uuid }}
              </el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>
        </el-menu>
      </el-scrollbar>
    </el-aside>

    <el-main>
      <div>
        <textarea id="log" readonly></textarea>
      </div>
    </el-main>
  </el-container>
</template>

<script>
import axios from 'axios';
import {SuccessFilled, CircleCloseFilled, QuestionFilled} from '@element-plus/icons-vue';
import {kamura_engine_url} from "@/utils/consts";

export default {
  name: 'KamuraRunner',
  data() {
    return {
      tasks: [],  // Store tasks with status and uuid
    };
  },
  created() {
    this.fetchAllTasks();
    this.startStatusUpdateTimer();
  },
  beforeUnmount() {
    this.stopStatusUpdateTimer();
  },
  methods: {
    startStatusUpdateTimer() {
      this.intervalId = setInterval(this.updateTaskStatuses, 2500);
    },
    stopStatusUpdateTimer() {
      if (this.intervalId) {
        clearInterval(this.intervalId);
        this.intervalId = null;
      }
    },
    async fetchAllTasks() {
      try {
        const response = await axios.get(`${kamura_engine_url}/getAllTasks`);
        if (response.data.success) {
          this.tasks = response.data.tasks.map(task => ({
            uuid: task,
            status: 'Loading'
          }));
        }
      } catch (error) {
        console.error('Error fetching tasks:', error);
      }
    },
    async updateTaskStatuses() {
      for (let task of this.tasks) {
        try {
          const statusResponse = await axios.post(`${kamura_engine_url}/getTaskStatus`, {uuid: task.uuid});
          if (statusResponse.data.success) {
            task.status = statusResponse.data.message;
          } else {
            task.status = 'Unknown';
          }
        } catch (error) {
          console.error('Error updating task status:', error);
        }
      }
    },
    async fetchTaskLog(uuid) {
      try {
        const response = await axios.post(`${kamura_engine_url}/getTaskLog`, {uuid});
        if (response.data.success) {
          let responseSpan = document.getElementById("log")
          responseSpan.innerHTML = response.data.message
        }
      } catch (error) {
        console.error('Error fetching task log:', error);
      }
    },
    getIconComponent(status) {
      switch (status) {
        case 'Succeed':
          return SuccessFilled;
        case 'Failed':
          return CircleCloseFilled;
        case 'Running':
          return QuestionFilled;
        default:
          return QuestionFilled;
      }
    },
    getStatusColor(status) {
      switch (status) {
        case 'Succeed':
          return 'green';
        case 'Failed':
          return 'red';
        case 'Running':
          return 'yellow';
        default:
          return 'gray';
      }
    },
    async addTaskToRunner(workload, workloadType) {
      try {
        const response = await axios.post(`${kamura_engine_url}/addTask`, {
          arch: 'simple',
          workload: workload,
          workload_type: workloadType
        });
        console.log('Response:', response.data);
      } catch (error) {
        console.error('Error:', error);
      }
    }
  }
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
