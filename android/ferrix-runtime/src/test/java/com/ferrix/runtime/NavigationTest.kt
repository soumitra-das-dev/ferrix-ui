package com.ferrix.runtime

import android.app.Application
import android.view.View
import androidx.test.core.app.ApplicationProvider
import com.ferrix.runtime.navigation.FerrixHeaderView
import com.ferrix.runtime.navigation.FerrixNavContainer
import com.ferrix.runtime.navigation.FerrixScreenView
import com.ferrix.runtime.navigation.FerrixTabBarView
import com.ferrix.runtime.protocol.Command
import com.ferrix.runtime.protocol.Patch
import com.ferrix.runtime.protocol.ViewType
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34])
class NavigationTest {

    private lateinit var app: Application
    private lateinit var rootLayout: android.widget.FrameLayout
    private lateinit var viewFactory: FerrixViewFactory
    private lateinit var renderer: FerrixRenderer

    @Before
    fun setup() {
        app = ApplicationProvider.getApplicationContext()
        rootLayout = android.widget.FrameLayout(app)
        viewFactory = FerrixViewFactory(app)
        renderer = FerrixRenderer(app, rootLayout)
    }

    @Test
    fun testFactoryCreatesNavigationViews() {
        val nav = viewFactory.createView(ViewType.NAV_CONTAINER)
        assertTrue(nav is FerrixNavContainer)

        val screen = viewFactory.createView(ViewType.SCREEN)
        assertTrue(screen is FerrixScreenView)

        val header = viewFactory.createView(ViewType.HEADER)
        assertTrue(header is FerrixHeaderView)

        val tabBar = viewFactory.createView(ViewType.TAB_BAR)
        assertTrue(tabBar is FerrixTabBarView)
    }

    @Test
    fun testNavContainerStackOperationsAndHeader() {
        val container = FerrixNavContainer(app)
        val screen1 = FerrixScreenView(app).apply {
            nodeId = 10L
            screenTitle = "Home"
        }
        val screen2 = FerrixScreenView(app).apply {
            nodeId = 20L
            screenTitle = "Details"
        }

        // Push Screen 1
        container.pushScreen(screen1, animation = 4) // None
        assertEquals(1, container.getStackSize())
        assertEquals(screen1, container.getActiveScreen())
        assertFalse(container.canGoBack())
        assertEquals("Home", container.headerView.titleView.text.toString())
        assertEquals(View.GONE, container.headerView.backButton.visibility)

        // Push Screen 2
        container.pushScreen(screen2, animation = 4)
        assertEquals(2, container.getStackSize())
        assertEquals(screen2, container.getActiveScreen())
        assertTrue(container.canGoBack())
        assertEquals("Details", container.headerView.titleView.text.toString())
        assertEquals(View.VISIBLE, container.headerView.backButton.visibility)

        // Pop Screen 2
        assertTrue(container.popScreen(animation = 4))
        assertEquals(1, container.getStackSize())
        assertEquals(screen1, container.getActiveScreen())
        assertFalse(container.canGoBack())
        assertEquals("Home", container.headerView.titleView.text.toString())
        assertEquals(View.GONE, container.headerView.backButton.visibility)
    }

    @Test
    fun testRendererAppliesNavigationCommands() {
        val containerId = 10L
        val screen1Id = 20L
        val screen2Id = 30L

        // Mount container and screens
        val patches = listOf(
            Patch.CreateNode(containerId, ViewType.NAV_CONTAINER),
            Patch.AppendChild(1L, containerId),
            Patch.CreateNode(screen1Id, ViewType.SCREEN),
            Patch.CreateNode(screen2Id, ViewType.SCREEN)
        )
        renderer.applyPatches(patches) { _, _, _ -> }

        val container = renderer.getView(containerId) as? FerrixNavContainer
        assertNotNull(container)

        // Command: Navigate to Screen 1
        renderer.applyCommands(listOf(
            Command.Navigate(containerId, screen1Id, animation = 4),
            Command.SetScreenOptions(screen1Id, "Dashboard", headerShown = true, presentation = 0)
        ))

        assertEquals(1, container!!.getStackSize())
        assertEquals("Dashboard", container.headerView.titleView.text.toString())

        // Command: Navigate to Screen 2
        renderer.applyCommands(listOf(
            Command.Navigate(containerId, screen2Id, animation = 4),
            Command.SetScreenOptions(screen2Id, "Settings", headerShown = true, presentation = 0)
        ))

        assertEquals(2, container.getStackSize())
        assertEquals("Settings", container.headerView.titleView.text.toString())
        assertTrue(container.canGoBack())

        // Command: GoBack
        renderer.applyCommands(listOf(
            Command.GoBack(containerId, animation = 4)
        ))

        assertEquals(1, container.getStackSize())
        assertEquals("Dashboard", container.headerView.titleView.text.toString())
        assertFalse(container.canGoBack())
    }
}
