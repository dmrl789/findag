import React, { useState } from 'react';

interface Annotation {
  id: string;
  x: number;
  y: number;
  text: string;
  color: string;
  type: 'point' | 'line' | 'text';
}

interface ChartAnnotationsProps {
  annotations: Annotation[];
  onAdd: (annotation: Annotation) => void;
  onRemove: (id: string) => void;
  onUpdate: (id: string, annotation: Annotation) => void;
}

const ChartAnnotations: React.FC<ChartAnnotationsProps> = ({
  annotations,
  onAdd,
  onRemove,
  onUpdate,
}) => {
  const [newAnnotation, setNewAnnotation] = useState({
    text: '',
    color: '#EF4444',
    type: 'point' as const,
  });
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editingAnnotation, setEditingAnnotation] = useState<Annotation | null>(null);

  const handleAddAnnotation = () => {
    if (!newAnnotation.text.trim()) return;

    const annotation: Annotation = {
      id: Date.now().toString(),
      x: Date.now(),
      y: Math.random() * 100,
      text: newAnnotation.text,
      color: newAnnotation.color,
      type: newAnnotation.type,
    };

    onAdd(annotation);
    setNewAnnotation({ text: '', color: '#EF4444', type: 'point' });
  };

  const handleEditStart = (annotation: Annotation) => {
    setEditingId(annotation.id);
    setEditingAnnotation({ ...annotation });
  };

  const handleEditSave = () => {
    if (editingAnnotation) {
      onUpdate(editingAnnotation.id, editingAnnotation);
      setEditingId(null);
      setEditingAnnotation(null);
    }
  };

  const handleEditCancel = () => {
    setEditingId(null);
    setEditingAnnotation(null);
  };

  const colorOptions = [
    { name: 'Red', value: '#EF4444' },
    { name: 'Blue', value: '#3B82F6' },
    { name: 'Green', value: '#10B981' },
    { name: 'Yellow', value: '#F59E0B' },
    { name: 'Purple', value: '#8B5CF6' },
    { name: 'Pink', value: '#EC4899' },
  ];

  const typeOptions = [
    { name: 'Point', value: 'point' },
    { name: 'Line', value: 'line' },
    { name: 'Text', value: 'text' },
  ];

  return (
    <div className="space-y-4">
      {/* Add New Annotation */}
      <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
        <h4 className="text-sm font-medium text-gray-900 dark:text-white mb-3">
          Add New Annotation
        </h4>
        <div className="space-y-3">
          <div>
            <label className="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">
              Text
            </label>
            <input
              type="text"
              value={newAnnotation.text}
              onChange={(e) => setNewAnnotation({ ...newAnnotation, text: e.target.value })}
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm"
              placeholder="Enter annotation text"
            />
          </div>
          
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">
                Color
              </label>
              <select
                value={newAnnotation.color}
                onChange={(e) => setNewAnnotation({ ...newAnnotation, color: e.target.value })}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm"
              >
                {colorOptions.map((color) => (
                  <option key={color.value} value={color.value}>
                    {color.name}
                  </option>
                ))}
              </select>
            </div>
            
            <div>
              <label className="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">
                Type
              </label>
              <select
                value={newAnnotation.type}
                onChange={(e) => setNewAnnotation({ ...newAnnotation, type: e.target.value as any })}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm"
              >
                {typeOptions.map((type) => (
                  <option key={type.value} value={type.value}>
                    {type.name}
                  </option>
                ))}
              </select>
            </div>
          </div>
          
          <button
            onClick={handleAddAnnotation}
            disabled={!newAnnotation.text.trim()}
            className="w-full px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed text-sm font-medium"
          >
            Add Annotation
          </button>
        </div>
      </div>

      {/* Existing Annotations */}
      <div>
        <h4 className="text-sm font-medium text-gray-900 dark:text-white mb-3">
          Existing Annotations ({annotations.length})
        </h4>
        
        {annotations.length === 0 ? (
          <p className="text-sm text-gray-500 dark:text-gray-400 text-center py-4">
            No annotations yet. Double-click on the chart to add one.
          </p>
        ) : (
          <div className="space-y-2">
            {annotations.map((annotation) => (
              <div
                key={annotation.id}
                className="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-3"
              >
                {editingId === annotation.id ? (
                  <div className="space-y-2">
                    <input
                      type="text"
                      value={editingAnnotation?.text || ''}
                      onChange={(e) => setEditingAnnotation(editingAnnotation ? { ...editingAnnotation, text: e.target.value } : null)}
                      className="w-full px-2 py-1 border border-gray-300 dark:border-gray-600 rounded text-sm bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                    />
                    <div className="flex space-x-2">
                      <select
                        value={editingAnnotation?.color || '#EF4444'}
                        onChange={(e) => setEditingAnnotation(editingAnnotation ? { ...editingAnnotation, color: e.target.value } : null)}
                        className="px-2 py-1 border border-gray-300 dark:border-gray-600 rounded text-sm bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                      >
                        {colorOptions.map((color) => (
                          <option key={color.value} value={color.value}>
                            {color.name}
                          </option>
                        ))}
                      </select>
                      <select
                        value={editingAnnotation?.type || 'point'}
                        onChange={(e) => setEditingAnnotation(editingAnnotation ? { ...editingAnnotation, type: e.target.value as any } : null)}
                        className="px-2 py-1 border border-gray-300 dark:border-gray-600 rounded text-sm bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                      >
                        {typeOptions.map((type) => (
                          <option key={type.value} value={type.value}>
                            {type.name}
                          </option>
                        ))}
                      </select>
                    </div>
                    <div className="flex space-x-2">
                      <button
                        onClick={handleEditSave}
                        className="px-3 py-1 bg-green-600 text-white rounded text-xs hover:bg-green-700"
                      >
                        Save
                      </button>
                      <button
                        onClick={handleEditCancel}
                        className="px-3 py-1 bg-gray-600 text-white rounded text-xs hover:bg-gray-700"
                      >
                        Cancel
                      </button>
                    </div>
                  </div>
                ) : (
                  <div className="flex items-center justify-between">
                    <div className="flex items-center space-x-2">
                      <div
                        className="w-3 h-3 rounded-full"
                        style={{ backgroundColor: annotation.color }}
                      />
                      <span className="text-sm text-gray-900 dark:text-white">
                        {annotation.text}
                      </span>
                      <span className="text-xs text-gray-500 dark:text-gray-400">
                        ({annotation.type})
                      </span>
                    </div>
                    <div className="flex space-x-1">
                      <button
                        onClick={() => handleEditStart(annotation)}
                        className="px-2 py-1 text-xs text-blue-600 hover:text-blue-700"
                      >
                        Edit
                      </button>
                      <button
                        onClick={() => onRemove(annotation.id)}
                        className="px-2 py-1 text-xs text-red-600 hover:text-red-700"
                      >
                        Remove
                      </button>
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default ChartAnnotations; 