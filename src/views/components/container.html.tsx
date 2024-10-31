import * as elements from 'typed-html';
import cn from 'classnames';
export type ContainerProps = {
  className?: string;
  title?: string;
  children?: elements.Children;
};
export function Container({ className = '', title, children }: ContainerProps) {
  return (
    <div class={cn(className, 'p-2 mx-2 border shadow')}>
      <h1>{title && <span class="font-semibold text-medium">{title}</span>}</h1>

      {children}
    </div>
  );
}
