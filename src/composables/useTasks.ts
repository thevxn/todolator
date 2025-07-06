import { invoke } from "@tauri-apps/api/core";
import { Ref, ref } from "vue";

type DateTimeString = string;

export interface ITask {
  id?: string;
  name: string;
  desc?: string;
  timestamp: DateTimeString;
}

export const useTasks = () => {
  const tasks = ref([]) as Ref<Array<ITask>>;
  const displayNewTaskModal = ref(false);
  const taskCreationError = ref(undefined) as Ref<string | undefined>;

  const loadTasks = async () => {
    try {
      tasks.value = await invoke("get_tasks");
      console.log("tasks loaded");

      console.log(tasks.value);
    } catch (e) {
      console.log(e);
    }
  };

  const saveTask = async (task: ITask) => {
    if (!task.name || !task.timestamp) {
      console.log("missing required attributes");
      taskCreationError.value = "Missing required attributes!";
      return;
    }
    try {
      const taskToSave = {
        id: task.id,
        name: task.name,
        desc: task.desc,
        timestamp: new Date(task.timestamp).toISOString()
      };
      console.log("Adding task: ", taskToSave);

      if (taskToSave.id) {
        // TODO: Implement update
      } else {
        // If the task does not have an ID, it's a create action
        await invoke("create_task", taskToSave);
        console.log("task created!");
      }

      tasks.value.push(task);

      toggleTaskModal();
    } catch (e) {
      console.log(e);
    }
  };

  const toggleTaskModal = () => {
    taskCreationError.value = undefined;
    displayNewTaskModal.value = !displayNewTaskModal.value;
  };

  return {
    tasks,
    displayNewTaskModal,
    taskCreationError,
    loadTasks,
    saveTask,
    toggleTaskModal
  };
};
