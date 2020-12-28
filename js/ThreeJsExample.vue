<template>
  <div id="main_content">
    <three-js-viewer
      :selected_objid="selected_id"
      @object_clicked="selected_id = $event"
    />
  </div>
</template>

<script>
import ThreeJsViewer from './ThreeJsViewer';

import 'bootstrap/dist/css/bootstrap.css';

export default {
	name: 'ThreeJsExample',
	components: {
		'three-js-viewer': ThreeJsViewer
	},
	data() {

		return {
			selected_id: null
		};

  }
};
</script>

<style>
html{
  margin: 0px;
  width: 100%;
  height: 100%;
  padding: 0px;
  overflow-y: hidden; /*important as otherwise size is somehow scrollable which is not desired */
}

body{
  margin: 0px;
  padding: 0px;
  height: 100%;
}

#main_content {
  height: 100%;
  width: 100%;
}
</style>
