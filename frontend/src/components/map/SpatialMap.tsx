import React, { useEffect, useMemo } from 'react';
import {
  ReactFlow,
  Background,
  Controls,
  useNodesState,
  useEdgesState,
  useReactFlow,
  ReactFlowProvider,
  Node,
  Edge,
  MarkerType,
} from '@xyflow/react';
import { LocationNode, LocationNodeData } from './LocationNode';
import { useGameStore } from '../../store';

const nodeTypes = {
  locationNode: LocationNode,
};

const SpatialMapInner: React.FC = () => {
  const { locations, locationEdges, currentLocationId, npcs, setSelectedLocationId } =
    useGameStore();
  const { setCenter } = useReactFlow();

  // Construct ReactFlow nodes
  const initialNodes: Node<LocationNodeData>[] = useMemo(() => {
    return locations.map((loc) => ({
      id: loc.id,
      type: 'locationNode',
      position: { x: loc.position_x, y: loc.position_y },
      data: {
        id: loc.id,
        name: loc.name,
        slug: loc.slug,
        description: loc.description,
        isCurrentLocation: loc.id === currentLocationId,
        npcsPresent: npcs.filter((n) => n.current_location_id === loc.id),
        onSelect: (locId) => setSelectedLocationId(locId),
      },
    }));
  }, [locations, currentLocationId, npcs, setSelectedLocationId]);

  // Construct ReactFlow edges
  const initialEdges: Edge[] = useMemo(() => {
    return locationEdges.map((edge) => ({
      id: edge.id,
      source: edge.source_location_id,
      target: edge.target_location_id,
      animated: !edge.is_locked,
      style: {
        stroke: edge.is_locked ? '#f43f5e' : '#64748b',
        strokeWidth: 2,
        strokeDasharray: edge.is_locked ? '5,5' : undefined,
      },
      label: edge.is_locked ? '🔒 Fermé' : undefined,
      labelStyle: { fill: '#fda4af', fontSize: 10, fontWeight: 600 },
      markerEnd: {
        type: MarkerType.ArrowClosed,
        color: edge.is_locked ? '#f43f5e' : '#64748b',
        width: 15,
        height: 15,
      },
    }));
  }, [locationEdges]);

  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  // Sync state updates
  useEffect(() => {
    setNodes(initialNodes);
  }, [initialNodes, setNodes]);

  useEffect(() => {
    setEdges(initialEdges);
  }, [initialEdges, setEdges]);

  // Smooth camera centering on current location when it changes
  useEffect(() => {
    const currentLoc = locations.find((l) => l.id === currentLocationId);
    if (currentLoc) {
      setCenter(currentLoc.position_x + 90, currentLoc.position_y + 40, {
        duration: 800,
        zoom: 1.2,
      });
    }
  }, [currentLocationId, locations, setCenter]);

  return (
    <div className="h-full w-full relative bg-[#0b0f19]">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        nodeTypes={nodeTypes}
        fitView
        fitViewOptions={{ padding: 0.3 }}
        minZoom={0.5}
        maxZoom={1.8}
        proOptions={{ hideAttribution: true }}
      >
        <Background color="#1e293b" gap={20} size={1} />
        <Controls className="!bg-slate-900 !border-slate-800 !fill-slate-400 [&>button]:!border-slate-800" />
      </ReactFlow>
    </div>
  );
};

export const SpatialMap: React.FC = () => {
  return (
    <ReactFlowProvider>
      <SpatialMapInner />
    </ReactFlowProvider>
  );
};
